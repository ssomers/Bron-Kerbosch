# coding: utf-8

from bron_kerbosch1 import bron_kerbosch1
from bron_kerbosch2 import bron_kerbosch2
from bron_kerbosch3 import bron_kerbosch3
from bron_kerbosch4 import bron_kerbosch4
from bron_kerbosch5 import bron_kerbosch5
from bron_kerbosch6 import bron_kerbosch6
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph
from graph import random_undirected_graph
from reporter import SimpleReporter
from publish import publish
import argparse
import random
import sys
import time
from typing import List

funcs = [
    bron_kerbosch1,
    bron_kerbosch2,
    bron_kerbosch3,
    bron_kerbosch4,
    bron_kerbosch5,
    bron_kerbosch6,
]


def bron_kerbosch(graph: Graph) -> List[List[int]]:
    first = None
    for func in funcs:
        reporter = SimpleReporter()
        func(
            graph=graph,
            clique=[],
            candidates=graph.connected_nodes(),
            excluded=set(),
            reporter=reporter)
        current = sorted(sorted(clique) for clique in reporter.cliques)
        if first is None:
            first = current
        elif first != current:
            raise ValueError(f'oops, {first} != {current}')
    assert first is not None
    return first


def bron_kerbosch_timed(graph: Graph):
    repeats = 10
    first = None
    times = []
    for func in funcs:
        begin = time.process_time()
        seconds = None
        diagnostic = None
        for _ in range(repeats):
            reporter = SimpleReporter()
            try:
                func(
                    graph=graph,
                    clique=[],
                    candidates=graph.connected_nodes(),
                    excluded=set(),
                    reporter=reporter)
            except RecursionError:
                diagnostic = 'recursed out'
                break
        seconds = (time.process_time() - begin) / repeats
        if diagnostic is None:
            current = sorted(sorted(clique) for clique in reporter.cliques)
            if first is None:
                first = current
            elif first != current:
                diagnostic = f'oops, {first} != {current}'
        if diagnostic is None:
            diagnostic = f'{seconds:5.2f}s, {reporter.cnt} recursive calls'
        else:
            seconds = None
        print(f'{func.__name__}: {diagnostic}')
        times.append(seconds)
    return times


def random_graph(order: int, size: int) -> Graph:
    begin = time.process_time()
    g = random_undirected_graph(order=order, size=size)
    seconds = time.process_time() - begin
    name = f'random of order {order}, size {size}'
    if order < 10:
        print(f'{name}: {g.adjacencies}')
    else:
        print(f'{name}: (generating took {seconds:.2f}s)')
    return g


def test_order_0():
    assert bron_kerbosch(Graph(adjacencies=[])) == []


def test_order_1():
    assert bron_kerbosch(Graph(adjacencies=[[]])) == []


def test_order_2_isolated():
    assert bron_kerbosch(Graph(adjacencies=[[], []])) == []


def test_order_2_connected():
    assert bron_kerbosch(Graph(adjacencies=[{1}, {0}])) == [[0, 1]]


def test_order_3_size_1():
    assert bron_kerbosch(Graph(adjacencies=[{1}, {0}, []])) == [[0, 1]]
    assert bron_kerbosch(Graph(adjacencies=[[], {2}, {1}])) == [[1, 2]]


def test_order_3_size_2():
    assert bron_kerbosch(Graph(adjacencies=[{1}, {0, 2}, {1}])) == [[0, 1],
                                                                    [1, 2]]


def test_order_3_size_3():
    assert bron_kerbosch(
        Graph(adjacencies=[{1, 2}, {0, 2}, {0, 1}])) == [[0, 1, 2]]


def test_order_4_size_2_isolated():
    assert bron_kerbosch(Graph(adjacencies=[{1, 2}, {0}, {0}, []])) == [[0, 1],
                                                                        [0, 2]]


def test_order_4_size_2_connected():
    assert bron_kerbosch(Graph(adjacencies=[{1}, {0}, {3}, {2}])) == [[0, 1],
                                                                      [2, 3]]


def test_order_4_size_4_p():
    assert bron_kerbosch(
        Graph(adjacencies=[{1}, {0, 2, 3}, {1, 3}, {1, 2}])) == [[0, 1],
                                                                 [1, 2, 3]]


def test_order_4_size_4_square():
    assert bron_kerbosch(
        Graph(adjacencies=[{1, 3}, {0, 2}, {1, 3}, {0, 2}])) == [
            [0, 1],
            [0, 3],
            [1, 2],
            [2, 3],
        ]


def test_order_4_size_4_square_diagonal():
    assert bron_kerbosch(
        Graph(adjacencies=[{1, 2, 3}, {0, 2}, {0, 1, 3}, {0, 2}])) == [
            [0, 1, 2],
            [0, 2, 3],
        ]


def test_sample():
    assert bron_kerbosch(Graph(adjacencies=SAMPLE_ADJACENCY_LIST)) == [
        [1, 2, 3, 4],
        [2, 3, 5],
        [5, 6, 7],
    ]


def test_random_graph():
    random.seed(19680516)
    random_graph(order=2, size=0)
    random_graph(order=3, size=0)
    random_graph(order=3, size=1)
    random_graph(order=3, size=2)
    random_graph(order=4, size=0)
    random_graph(order=4, size=1)
    random_graph(order=4, size=2)
    random_graph(order=4, size=3)
    random_graph(order=4, size=4)
    random_graph(order=4, size=5)


def bk(order: int, sizes):
    times_per_size = []
    for size in sizes:
        random.seed(seed)
        g = random_graph(order=order, size=size)
        times_per_size.append(bron_kerbosch_timed(g))
    publish(
        language="python3",
        num_funcs=len(funcs),
        order=order,
        sizes=sizes,
        times_per_size=times_per_size)


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
        bk(order=int(args.order), sizes=[int(size) for size in args.size])
    else:
        assert False, "Run with -O for meaningful measurements"
        bk(order=50, sizes=range(750, 1_000, 10))  # max 1225
        bk(order=10_000,
           sizes=list(range(1_000, 10_000, 1_000)) + list(
               range(10_000, 100_000, 10_000)))
    print(f"random seed was {seed}")
