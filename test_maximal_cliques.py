# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2, bron_kerbosch4, bron_kerbosch5
from bronker_bosch3 import bron_kerbosch3, bron_kerbosch6
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph
from reporter import SimpleReporter
import argparse
import random
import sys
import time
from typing import List, Set

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


def bron_kerbosch_manual(graph: Graph):
    repeats = 7
    first = None
    for func in funcs:
        begin = time.process_time()
        result = None
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
                result = 'recursed out'
        if result is None:
            seconds = (time.process_time() - begin) / repeats
            current = sorted(sorted(clique) for clique in reporter.cliques)
            if first is None:
                first = current
            elif first != current:
                result = f'oops, {first} != {current}'
            if result is None:
                result = f'{seconds:5.2f}s, {reporter.cnt} recursive calls'
        print(f'{func.__name__}: {result}')
    if first is None:
        raise ValueError


def random_graph(order: int, size: int) -> Graph:
    fully_meshed_size = order * (order - 1) // 2
    if size > fully_meshed_size:
        raise ValueError(
            f"{order} nodes accommodate at most {fully_meshed_size} edges")
    begin = time.process_time()
    name = f'random_of_order_{order}_size_{size}'
    vertices = range(order)
    unsaturated_vertices = list(vertices)
    adjacency_sets: List[Set[int]] = [set() for _ in range(order)]
    adjacency_complements: List[Set[int]] = [set()] * order
    for _ in range(size):
        v = random.choice(unsaturated_vertices)
        assert len(adjacency_sets[v]) < order - 1
        if adjacency_complements[v]:
            w = random.sample(adjacency_complements[v], 1)[0]
        else:
            w = v
            while w == v or w in adjacency_sets[v]:
                w = random.choice(unsaturated_vertices)
        assert v != w
        assert w not in adjacency_sets[v]
        assert v not in adjacency_sets[w]
        for x, y in [(v, w), (w, v)]:
            adjacency_sets[x].add(y)
            neighbours = len(adjacency_sets[x])
            if neighbours == order - 1:
                unsaturated_vertices.remove(x)
            elif neighbours == order // 2:
                # start using adjacency complement
                assert not adjacency_complements[x]
                adjacency_complements[x] = (
                    set(unsaturated_vertices) - {x} - adjacency_sets[x])
            elif neighbours > order // 2:
                adjacency_complements[x].remove(y)
    seconds = time.process_time() - begin
    g = Graph(adjacencies=adjacency_sets)
    assert g.order == order
    assert g.size() == size
    contents = ''
    if order < 10:
        contents = ' ' + repr(adjacency_sets)
    print(f'{name}: (spent {seconds:.2f}s generating{contents})')
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


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="test Bron-Kerbosch implementations " +
        "on some random graphs of specified or default dimensions")
    parser.add_argument('--seed', nargs=1)
    parser.add_argument('order', nargs='?')
    parser.add_argument('size', nargs='*')
    args = parser.parse_args(sys.argv[1:])
    if args.seed:
        args.seed = int(args.seed[0])
    else:
        args.seed = random.randrange(1 << 32)
    print(f"random seed {args.seed}")
    random.seed(args.seed)
    if args.order is not None and args.size is not None:
        size_by_order = {int(args.order): [int(size) for size in args.size]}
    else:
        size_by_order = {
            26: [220, 240, 260, 280, 300],  # max 325
            82: [1800, 2000, 2200],  # max 3321
            10000: [10000, 20000, 30000, 40000, 50000, 60000],
        }
    for order, sizes in size_by_order.items():
        for size in sizes:
            bron_kerbosch_manual(random_graph(order=order, size=size))
