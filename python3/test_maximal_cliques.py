# coding: utf-8

from bron_kerbosch1 import bron_kerbosch1
from bron_kerbosch2 import bron_kerbosch2
from bron_kerbosch3 import bron_kerbosch3
from bron_kerbosch1o import bron_kerbosch1o
from bron_kerbosch2_rp import bron_kerbosch2_rp
from bron_kerbosch2_gp import bron_kerbosch2_gp
from bron_kerbosch2_gpx import bron_kerbosch2_gpx
from bron_kerbosch3o import bron_kerbosch3o
from bron_kerbosch3n import bron_kerbosch3n
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph, random_undirected_graph, Vertex
from reporter import SimpleReporter
from stats import SampleStatistics
from publish import publish

import argparse
import pytest
import random
import sys
import time
from typing import List, Set

FUNCS = [
    bron_kerbosch1,
    bron_kerbosch2,
    bron_kerbosch3,
    bron_kerbosch1o,
    bron_kerbosch2_rp,
    bron_kerbosch2_gp,
    bron_kerbosch2_gpx,
    bron_kerbosch3o,
    bron_kerbosch3n,
]

FUNC_NAMES = [
    "Ver1",
    "Ver2",
    "Ver3",
    "Ver1+",
    "Ver2_RP",
    "Ver2_GP",
    "Ver2_GPX",
    "Ver3+",
    "Ver3-",
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


def bron_kerbosch_timed(graph: Graph, samples: int):
    first = None
    times = [SampleStatistics() for _ in range(len(FUNCS))]
    for sample in range(samples):
        for func_index, func in enumerate(FUNCS):
            reporter = SimpleReporter()
            begin = time.process_time()
            try:
                func(graph=graph, reporter=reporter)
            except RecursionError:
                print(f"  {FUNC_NAMES[func_index]} recursed out")
            secs = time.process_time() - begin
            if secs >= 1.0:
                print(f"  {FUNC_NAMES[func_index]:8}: {secs:5.2}s")
            if sample < 2:
                current = sorted(sorted(clique) for clique in reporter.cliques)
                if first is None:
                    if not are_maximal(current):
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
def test_order_3_size_1(func):
    assert bkf(func=func, adjacencies=[{1}, {0}, []]) == [[0, 1]]
    assert bkf(func=func, adjacencies=[[], {2}, {1}]) == [[1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_2(func):
    assert bkf(func=func, adjacencies=[{1}, {0, 2}, {1}]) == [[0, 1], [1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_3_size_3(func):
    assert bkf(func=func, adjacencies=[{1, 2}, {0, 2}, {0, 1}]) == [[0, 1, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_2_isolated(func):
    assert bkf(
        func=func, adjacencies=[{1, 2}, {0}, {0}, []]) == [[0, 1], [0, 2]]


@pytest.mark.parametrize("func", FUNCS)
def test_order_4_size_2_connected(func):
    assert bkf(func=func, adjacencies=[{1}, {0}, {3}, {2}]) == [[0, 1], [2, 3]]


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
        func=func, adjacencies=[{1, 2, 3}, {0, 2, 3}, {0, 1, 3},
                                {0, 1, 2}]) == [
                                    [0, 1, 2, 3],
                                ]


@pytest.mark.parametrize("func", FUNCS)
def test_sample(func):
    assert bkf(
        func=func, adjacencies=SAMPLE_ADJACENCY_LIST) == [
            [1, 2, 3, 4],
            [2, 3, 5],
            [5, 6, 7],
        ]


def bk(orderstr: str, sizes):
    if orderstr.endswith('M'):
        order = int(orderstr[:-1]) * 1_000_000
    elif orderstr.endswith('k'):
        order = int(orderstr[:-1]) * 1_000
    else:
        order = int(orderstr)
    stats_per_size = []
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
        stats_per_size.append(bron_kerbosch_timed(g, samples=5))
    if len(sizes) > 1:
        publish(
            language="python3",
            orderstr=orderstr,
            func_names=FUNC_NAMES,
            sizes=sizes,
            stats_per_size=stats_per_size)


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
    if args.order is not None and args.size is not None:
        bk(orderstr=args.order, sizes=[int(size) for size in args.size])
    else:
        assert False, "Run with -O for meaningful measurements"
        bk(orderstr="100", sizes=range(2_000, 3_001, 50))  # max 4_950
        time.sleep(10)
        bk(orderstr="10k",
           sizes=list(range(1_000, 10_000, 1_000)) + list(
               range(10_000, 200_001, 10_000)))
    print(f"random seed was {seed}")
