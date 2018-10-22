# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2, bron_kerbosch4
from bronker_bosch3 import bron_kerbosch3, bron_kerbosch6
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import Graph
from reporter import SimpleReporter
import random
import time

funcs = {
    bron_kerbosch1: 100,
    bron_kerbosch2: 100,
    bron_kerbosch4: 100,
    bron_kerbosch3: 1,
    bron_kerbosch6: 1,
}


def bron_kerbosch(graph: Graph):
    first = None
    for func, repeat in funcs.items():
        begin = time.process_time()
        result = None
        for _ in range(repeat):
            reporter = SimpleReporter()
            try:
                func(
                    NEIGHBORS=graph.adjacencies,
                    clique=[],
                    candidates=graph.connected_nodes(),
                    excluded=set(),
                    reporter=reporter)
            except RecursionError:
                result = f'recursed out (adjacencies={graph.adjacencies})'
        if result is None:
            seconds = (time.process_time() - begin) / repeat
            current = sorted(sorted(clique) for clique in reporter.cliques)
            if first is None:
                first = current
            if first != current:
                result = f'oops, {first} != {current}'
            elif graph.name:
                result = f'{seconds:7.3f}s, {reporter.cnt} recursive calls'
        if result:
            print(f'{func.__name__}@{graph.name}: {result}')
    return first


def random_graph(order, size):
    assert size < order * (order - 1) // 2
    begin = time.process_time()
    name = f'random_of_order_{order}_size_{size}'
    vertices = range(1, order + 1)
    unsaturated_vertices = list(vertices)
    adjacency_sets = [None] + [set() for _ in range(order)]
    adjacency_complements = [None] + [None for _ in range(order)]
    for _ in range(size):
        v = random.choice(unsaturated_vertices)
        assert len(adjacency_sets[v]) < order - 1
        if adjacency_complements[v]:
            w = random.choice(adjacency_complements[v])
        else:
            w = v
            while w == v or w in adjacency_sets[v]:
                w = random.choice(unsaturated_vertices)
        assert v != w
        assert w not in adjacency_sets[v]
        assert v not in adjacency_sets[w]
        adjacency_sets[v].add(w)
        adjacency_sets[w].add(v)
        if adjacency_complements[v]:
            adjacency_complements[v].remove(w)
        if adjacency_complements[w]:
            adjacency_complements[w].remove(v)
        for x in (v, w):
            if len(adjacency_sets[x]) == order - 1:
                unsaturated_vertices.remove(x)
            elif len(adjacency_sets[x]) == order // 2:
                assert adjacency_complements[x] is None
                other_vertices = list(unsaturated_vertices)
                other_vertices.remove(x)
                for y in adjacency_sets[x]:
                    other_vertices.remove(y)
                adjacency_complements[x] = other_vertices
    seconds = time.process_time() - begin
    print(f'spent {seconds:.2f}s generating {name}')
    g = Graph(name=name, adjacencies=adjacency_sets)
    assert g.order == order
    return g


if __name__ == '__main__':
    size_by_order = {
        10: [20],
        100: [200],
        1000: [2000],
        10000: [5000, 10000, 15000, 20000],
    }
    for order, sizes in size_by_order.items():
        for size in sizes:
            bron_kerbosch(random_graph(order=order, size=size))


def test_empty():
    assert bron_kerbosch(Graph(adjacencies=[None])) == []


def test_one():
    assert bron_kerbosch(Graph(adjacencies=[None, []])) == []


def test_two_isolated():
    assert bron_kerbosch(Graph(adjacencies=[None, [], []])) == []


def test_two_connected():
    assert bron_kerbosch(Graph(adjacencies=[None, [2], [1]])) == [[1, 2]]


def test_sample():
    assert bron_kerbosch(Graph(adjacencies=SAMPLE_ADJACENCY_LIST)) == [
        [1, 2, 3, 4],
        [2, 3, 5],
        [5, 6, 7],
    ]
