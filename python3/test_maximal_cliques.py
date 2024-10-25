# coding: utf-8

import bron_kerbosch1a
import bron_kerbosch1b
import bron_kerbosch2a_gp
import bron_kerbosch2b_gp
import bron_kerbosch2b_gpx
import bron_kerbosch3_gp
import bron_kerbosch3_gpx
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph, Vertex
from random_graph import to_int, read_random_graph
from reporter import CollectingReporter, CountingReporter, Reporter
from stats import SampleStatistics
from publish import publish

import argparse
import itertools
from math import isnan
import pytest
import sys
import time
from typing import Callable, Iterable, List, Set

FUNC = Callable[[Graph, Reporter], None]

FUNCS: List[FUNC] = [
    bron_kerbosch1a.explore,
    bron_kerbosch1b.explore,
    bron_kerbosch2a_gp.explore,
    bron_kerbosch2b_gp.explore,
    bron_kerbosch2b_gpx.explore,
    bron_kerbosch3_gp.explore,
    bron_kerbosch3_gpx.explore,
]

FUNC_NAMES = [
    "Ver1",
    "Ver1½",
    "Ver2-GP",
    "Ver2½-GP",
    "Ver2½-GPX",
    "Ver3½-GP",
    "Ver3½-GPX",
]


def are_maximal(cliques: List[List[Vertex]]) -> bool:
    for j, clique2 in enumerate(cliques):
        if j % 1000 == 0:
            print(f"checking maximality {j}/{len(cliques)}")
        for i, clique1 in enumerate(cliques[:j]):
            if clique1[: len(clique2)] == clique2[: len(clique1)]:
                return False
    print("checked maximality")
    return True


def bron_kerbosch_timed(
    graph: Graph, clique_count: int, func_indices: List[int], timed_samples: int
) -> List[SampleStatistics]:
    first = None
    times = [SampleStatistics() for _ in range(len(FUNCS))]
    for sample in range(timed_samples + 1):
        for func_index in func_indices:
            func = FUNCS[func_index]
            func_name = FUNC_NAMES[func_index]
            collecting_reporter = CollectingReporter()
            counting_reporter = CountingReporter()
            begin = time.perf_counter()
            try:
                if sample == 0:
                    func(graph, collecting_reporter)
                else:
                    func(graph, counting_reporter)
            except RecursionError:
                print(f"{func_name} recursed out!")
            secs = time.perf_counter() - begin
            if sample == 0:
                if secs >= 3.0:
                    print(f"  {func_name:<8}: {secs:6.3f}s")
                current = sorted(
                    sorted(clique) for clique in collecting_reporter.cliques
                )
                if first is None:
                    if len(current) != clique_count:
                        print(
                            f"{func_name}: expected {clique_count},"
                            f" obtained {len(current)} cliques!"
                        )
                    if graph.order < 100 and not are_maximal(current):
                        print(f"  {func_name} not maximal")
                    first = current
                elif first != current:
                    print(
                        f"{func_name}: "
                        + f"expected {len(first)} cliques, "
                        + f"obtained {len(current)} different cliques!"
                    )
            else:
                if counting_reporter.cliques != clique_count:
                    print(
                        f"{func_name}: expected {clique_count},"
                        f" obtained {counting_reporter.cliques} cliques!"
                    )
                times[func_index].put(secs)
    return times


def bkf(func: FUNC, adjacencies: List[Set[Vertex]]) -> List[List[Vertex]]:
    reporter = CollectingReporter()
    func(Graph(adjacencies=adjacencies), reporter)
    return sorted(sorted(clique) for clique in reporter.cliques)


@pytest.mark.parametrize("func", FUNCS)
def test_order_0(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_1(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[set()]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_2_isolated(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[set(), set()]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_2_connected(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0}]) == [[0, 1]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_left(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0}, set()]) == [[0, 1]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_long(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{2}, set(), {0}]) == [[0, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_right(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[set(), {2}, {1}]) == [[1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_2(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0, 2}, {1}]) == [[0, 1], [1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_3(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1, 2}, {0, 2}, {0, 1}]) == [[0, 1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_2(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0}, {3}, {2}]) == [[0, 1], [2, 3]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_3_bus(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0, 2}, {1, 3}, {2}]) == [
        [0, 1],
        [1, 2],
        [2, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_3_star(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1, 2, 3}, {0}, {0}, {0}]) == [
        [0, 1],
        [0, 2],
        [0, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_4_p(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1}, {0, 2, 3}, {1, 3}, {1, 2}]) == [
        [0, 1],
        [1, 2, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_4_square(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1, 3}, {0, 2}, {1, 3}, {0, 2}]) == [
        [0, 1],
        [0, 3],
        [1, 2],
        [2, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_5(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=[{1, 2, 3}, {0, 2}, {0, 1, 3}, {0, 2}]) == [
        [0, 1, 2],
        [0, 2, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_6(func: FUNC) -> None:
    assert bkf(
        func=func,
        adjacencies=[
            {1, 2, 3},
            {0, 2, 3},
            {0, 1, 3},
            {0, 1, 2},
        ],
    ) == [
        [0, 1, 2, 3],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_5_penultimate(func: FUNC) -> None:
    assert bkf(
        func=func,
        adjacencies=[
            {1, 2, 3, 4},
            {0, 2, 3, 4},
            {0, 1, 3, 4},
            {0, 1, 2},
            {0, 1, 2},
        ],
    ) == [
        [0, 1, 2, 3],
        [0, 1, 2, 4],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_sample(func: FUNC) -> None:
    assert bkf(func=func, adjacencies=SAMPLE_ADJACENCY_LIST) == [
        [1, 2, 3, 4],
        [2, 3, 5],
        [5, 6, 7],
    ]


@pytest.mark.parametrize("func", FUNCS)
def test_bigger(func: FUNC) -> None:
    assert bkf(
        func=func,
        adjacencies=[
            {1, 2, 3, 4, 6, 7},
            {0, 3, 6, 7, 8, 9},
            {0, 3, 5, 7, 8, 9},
            {0, 1, 2, 4, 9},
            {0, 3, 6, 7, 9},
            {2, 6},
            {0, 1, 4, 5, 9},
            {0, 1, 2, 4, 9},
            {1, 2},
            {1, 2, 3, 4, 6, 7},
        ],
    ) == [
        [0, 1, 3],
        [0, 1, 6],
        [0, 1, 7],
        [0, 2, 3],
        [0, 2, 7],
        [0, 3, 4],
        [0, 4, 6],
        [0, 4, 7],
        [1, 3, 9],
        [1, 6, 9],
        [1, 7, 9],
        [1, 8],
        [2, 3, 9],
        [2, 5],
        [2, 7, 9],
        [2, 8],
        [3, 4, 9],
        [4, 6, 9],
        [4, 7, 9],
        [5, 6],
    ]


def bk(
    orderstr: str, sizes: Iterable[int], func_indices: List[int], timed_samples: int
) -> None:
    order = to_int(orderstr)
    stats_per_func_by_size = {}
    for size in sizes:
        begin = time.process_time()
        g, clique_count = read_random_graph(orderstr=orderstr, size=size)
        secs = time.process_time() - begin
        name = f"random of order {orderstr}, size {size}, {clique_count} cliques:"
        if order < 10:
            print(f"{name} {g.adjacencies}")
        else:
            print(f"{name} (creating took {secs:.3f}s)")
        stats = bron_kerbosch_timed(
            g,
            clique_count=clique_count,
            func_indices=func_indices,
            timed_samples=timed_samples,
        )
        for func_index, func_name in enumerate(FUNC_NAMES):
            mean = stats[func_index].mean()
            if not isnan(mean):
                reldev = stats[func_index].deviation() / mean
                print(f"  {func_name:<8}: {mean:6.3f}s ± {reldev:.0%}")
        stats_per_func_by_size[size] = stats
    if len(stats_per_func_by_size) > 1:
        publish(
            language="python311",
            orderstr=orderstr,
            case_names=FUNC_NAMES,
            stats_per_func_by_size=stats_per_func_by_size,
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="test Bron-Kerbosch implementations "
        + "on some random graphs of specified or default dimensions"
    )
    parser.add_argument("--seed", nargs=1)
    parser.add_argument("order", nargs="?")
    parser.add_argument("size", nargs="*")
    args = parser.parse_args(sys.argv[1:])
    if args.seed:
        seed = int(args.seed[0])
    else:
        seed = 19680516
    all_func_indices = list(range(len(FUNCS)))
    most_func_indices = list(range(2, len(FUNCS)))
    mt_func_indices = list(range(3, len(FUNCS)))
    if args.order is not None and args.size is not None:
        bk(
            orderstr=args.order,
            sizes=[int(size) for size in args.size],
            func_indices=all_func_indices,
            timed_samples=0,
        )
    else:
        assert False, "Run with -O for meaningful measurements"
        bk(
            orderstr="100",
            sizes=range(2_000, 3_001, 50),  # max 4_950
            func_indices=all_func_indices,
            timed_samples=5,
        )
        time.sleep(7)
        bk(
            orderstr="10k",
            sizes=itertools.chain(
                range(10_000, 100_000, 10_000), range(100_000, 200_001, 25_000)
            ),
            func_indices=most_func_indices,
            timed_samples=3,
        )
        time.sleep(7)
        bk(
            orderstr="1M",
            sizes=itertools.chain(
                range(500_000, 2_000_000, 250_000),
                range(2_000_000, 3_000_001, 1_000_000),
            ),
            func_indices=mt_func_indices,
            timed_samples=3,
        )
