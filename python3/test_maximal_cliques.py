# coding: utf-8

from bron_kerbosch1 import bron_kerbosch1
from bron_kerbosch1o import bron_kerbosch1o
from bron_kerbosch2 import bron_kerbosch2
from bron_kerbosch2_g import bron_kerbosch2_g
from bron_kerbosch2_gp import bron_kerbosch2_gp
from bron_kerbosch2_gpx import bron_kerbosch2_gpx
from bron_kerbosch2_rp import bron_kerbosch2_rp
from bron_kerbosch3 import bron_kerbosch3
from bron_kerbosch3_gp import bron_kerbosch3_gp
from bron_kerbosch3_gpx import bron_kerbosch3_gpx
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph, random_undirected_graph, Vertex
from reporter import SimpleReporter
from stats import SampleStatistics
from publish import publish

import argparse
import itertools
import pytest
import random
import sys
import time
from typing import Iterable, List, Set

FUNCS = [
    bron_kerbosch1,
    bron_kerbosch1o,
    bron_kerbosch2,
    bron_kerbosch2_g,
    bron_kerbosch2_gp,
    bron_kerbosch2_gpx,
    bron_kerbosch2_rp,
    bron_kerbosch3,
    bron_kerbosch3_gp,
    bron_kerbosch3_gpx,
]

FUNC_NAMES = [
    "Ver1",
    "Ver1+",
    "Ver2+",
    "Ver2+G",
    "Ver2+GP",
    "Ver2+GPX",
    "Ver2+RP",
    "Ver3+",
    "Ver3+GP",
    "Ver3+GPX",
]


def are_maximal(cliques: List[List[Vertex]]):
    for j, clique2 in enumerate(cliques):
        if j % 1000 == 0:
            print(f"checking maximality {j}/{len(cliques)}")
        for i, clique1 in enumerate(cliques[:j]):
            if clique1[:len(clique2)] == clique2[:len(clique1)]:
                return False
    print("checked maximality")
    return True


def bron_kerbosch_timed(graph: Graph, func_indices: List[int], samples: int):
    first = None
    times = [SampleStatistics() for _ in range(len(FUNCS))]
    for sample in range(samples):
        for func_index in func_indices:
            func = FUNCS[func_index]
            reporter = SimpleReporter()
            begin = time.process_time()
            try:
                func(graph=graph, reporter=reporter)
            except RecursionError:
                print(f"  {FUNC_NAMES[func_index]} recursed out")
            secs = time.process_time() - begin
            if secs >= 3.0:
                print(f"  {FUNC_NAMES[func_index]:8}: {secs:5.2f}s")
            if sample < 2:
                current = sorted(sorted(clique) for clique in reporter.cliques)
                if first is None:
                    if graph.order < 100 and not are_maximal(current):
                        print(f"  {FUNC_NAMES[func_index]:8} not maximal")
                    first = current
                elif first != current:
                    print(f"  {FUNC_NAMES[func_index]}: " +
                          f"expected {len(first)} cliques, " +
                          f"obtained {len(current)} different cliques")
            times[func_index].put(secs)
    return times


def bkf(func, adjacencies: List[Set[Vertex]]) -> List[List[Vertex]]:
    reporter = SimpleReporter()
    func(graph=Graph(adjacencies=adjacencies), reporter=reporter)
    return sorted(sorted(clique) for clique in reporter.cliques)


@pytest.mark.parametrize("func", FUNCS)
def test_order_0(func):
    assert bkf(func=func, adjacencies=[]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_1(func):
    assert bkf(func=func, adjacencies=[[]]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_2_isolated(func):
    assert bkf(func=func, adjacencies=[[], []]) == []


@pytest.mark.parametrize("func", FUNCS)
def test_order_2_connected(func):
    assert bkf(func=func, adjacencies=[{1}, {0}]) == [[0, 1]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_left(func):
    assert bkf(func=func, adjacencies=[{1}, {0}, []]) == [[0, 1]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_long(func):
    assert bkf(func=func, adjacencies=[{2}, [], {0}]) == [[0, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_1_right(func):
    assert bkf(func=func, adjacencies=[[], {2}, {1}]) == [[1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_2(func):
    assert bkf(func=func, adjacencies=[{1}, {0, 2}, {1}]) == [[0, 1], [1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_3(func):
    assert bkf(func=func, adjacencies=[{1, 2}, {0, 2}, {0, 1}]) == [[0, 1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_2(func):
    assert bkf(func=func, adjacencies=[{1}, {0}, {3}, {2}]) == [[0, 1], [2, 3]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_3_bus(func):
    assert bkf(
        func=func, adjacencies=[{1}, {0, 2}, {1, 3}, {2}]) == [
            [0, 1],
            [1, 2],
            [2, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_3_star(func):
    assert bkf(
        func=func, adjacencies=[{1, 2, 3}, {0}, {0}, {0}]) == [
            [0, 1],
            [0, 2],
            [0, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_4_p(func):
    assert bkf(
        func=func, adjacencies=[{1}, {0, 2, 3}, {1, 3}, {1, 2}]) == [
            [0, 1],
            [1, 2, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_4_square(func):
    assert bkf(
        func=func, adjacencies=[{1, 3}, {0, 2}, {1, 3}, {0, 2}]) == [
            [0, 1],
            [0, 3],
            [1, 2],
            [2, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_5(func):
    assert bkf(
        func=func, adjacencies=[{1, 2, 3}, {0, 2}, {0, 1, 3}, {0, 2}]) == [
            [0, 1, 2],
            [0, 2, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_6(func):
    assert bkf(
        func=func, adjacencies=[
            {1, 2, 3},
            {0, 2, 3},
            {0, 1, 3},
            {0, 1, 2},
        ]) == [
            [0, 1, 2, 3],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_order_5_penultimate(func):
    assert bkf(
        func=func,
        adjacencies=[
            {1, 2, 3, 4},
            {0, 2, 3, 4},
            {0, 1, 3, 4},
            {0, 1, 2},
            {0, 1, 2},
        ]) == [
            [0, 1, 2, 3],
            [0, 1, 2, 4],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_sample(func):
    assert bkf(
        func=func, adjacencies=SAMPLE_ADJACENCY_LIST) == [
            [1, 2, 3, 4],
            [2, 3, 5],
            [5, 6, 7],
        ]


@pytest.mark.parametrize("func", FUNCS)
def test_bigger(func):
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
        ]) == [
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


def bk(orderstr: str, sizes: Iterable[int], func_indices: List[int],
       samples: int):
    if orderstr.endswith('M'):
        order = int(orderstr[:-1]) * 1_000_000
    elif orderstr.endswith('k'):
        order = int(orderstr[:-1]) * 1_000
    else:
        order = int(orderstr)
    stats_per_func_by_size = {}
    for size in sizes:
        random.seed(seed)
        begin = time.process_time()
        g = random_undirected_graph(order=order, size=size)
        secs = time.process_time() - begin
        name = f"random of order {order}, size {size}"
        if order < 10:
            print(f"{name}: {g.adjacencies}")
        else:
            print(f"{name} (generating took {secs:.2f}s)")
        stats = bron_kerbosch_timed(
            g, func_indices=func_indices, samples=samples)
        for func_index, func_name in enumerate(FUNC_NAMES):
            mean = stats[func_index].mean()
            dev = stats[func_index].deviation()
            print(f"  {func_name:<8}: {mean:5.2f}s ±{dev:5.2f}s")
        stats_per_func_by_size[size] = stats
    if len(stats_per_func_by_size) > 1:
        publish(
            language="python3",
            orderstr=orderstr,
            case_names=FUNC_NAMES,
            stats_per_func_by_size=stats_per_func_by_size)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="test Bron-Kerbosch implementations " +
        "on some random graphs of specified or default dimensions")
    parser.add_argument('--seed', nargs=1)
    parser.add_argument('order', nargs='?')
    parser.add_argument('size', nargs='*')
    args = parser.parse_args(sys.argv[1:])
    if args.seed:
        seed = int(args.seed[0])
    else:
        seed = 19680516
    all_func_indices = list(range(len(FUNCS)))
    if args.order is not None and args.size is not None:
        bk(orderstr=args.order,
           sizes=[int(size) for size in args.size],
           func_indices=all_func_indices,
           samples=1)
    else:
        assert False, "Run with -O for meaningful measurements"
        bk(
            orderstr="100",
            sizes=range(2_000, 3_001, 50),  # max 4_950
            func_indices=all_func_indices,
            samples=5)
        time.sleep(7)
        bk(
            orderstr="10k",
            sizes=range(100_000, 800_001, 100_000),  # max 49_995_000
            func_indices=all_func_indices,
            samples=3)
        time.sleep(7)
        bk(orderstr="1M",
           sizes=itertools.chain(
               range(200_000, 1_000_000, 200_000),
               range(1_000_000, 5_000_001, 1_000_000)),
           func_indices=list(range(1, len(FUNCS))),
           samples=3)
    print(f"random seed was {seed}")
