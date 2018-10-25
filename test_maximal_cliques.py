# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2, bron_kerbosch4
from bronker_bosch3 import bron_kerbosch3, bron_kerbosch6
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph
from reporter import SimpleReporter
import random
import sys
import time
from typing import List, Set

funcs = [
    bron_kerbosch1,
    bron_kerbosch2,
    bron_kerbosch4,
    bron_kerbosch3,
    bron_kerbosch6,
]


def bron_kerbosch(graph: Graph) -> List[List[int]]:
    assert not graph.name
    first = None
    for func in funcs:
        reporter = SimpleReporter()
        func(
            NEIGHBORS=graph.adjacencies,
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
    assert graph.name
    first = None
    for func in funcs:
        begin = time.process_time()
        result = None
        for _ in range(repeats):
            reporter = SimpleReporter()
            try:
                func(
                    NEIGHBORS=graph.adjacencies,
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
        print(f'{func.__name__}@{graph.name}: {result}')
    if first is None:
        raise ValueError


def random_graph(order: int, size: int) -> Graph:
    assert size < order * (order - 1) // 2
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
    contents = ''
    if order < 10:
        contents = ' ' + repr(adjacency_sets)
    print(f'spent {seconds:.2f}s generating {name}{contents}')
    g = Graph(name=name, adjacencies=adjacency_sets)
    assert g.order == order
    assert g.size() == size
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


def test_sample():
    assert bron_kerbosch(Graph(adjacencies=SAMPLE_ADJACENCY_LIST)) == [
        [1, 2, 3, 4],
        [2, 3, 5],
        [5, 6, 7],
    ]


def test_random_graph():
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
    random.seed(19680516)
    if len(sys.argv) > 1:
        size_by_order = {
            int(sys.argv[1]): [int(size) for size in sys.argv[2:]]
        }
    else:
        size_by_order = {
            10: [20],
            100: [200],
            1000: [2000],
            10000: [5000, 10000, 15000, 20000],
        }
    for order, sizes in size_by_order.items():
        for size in sizes:
            bron_kerbosch_manual(random_graph(order=order, size=size))
