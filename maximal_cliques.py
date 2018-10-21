# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2
from bronker_bosch3 import bron_kerbosch3, bron_kerbosch4
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import Graph
from reporter import Reporter
import random
import time

funcs = [bron_kerbosch1, bron_kerbosch2, bron_kerbosch3, bron_kerbosch4]


def test_on_graph(graph: Graph):
    graph.validate()
    first = None
    for func in funcs:
        report = Reporter('')
        begin = time.process_time()
        try:
            func(
                NEIGHBORS=graph.adjacency_list,
                clique=[],
                candidates=set(graph.nodes),
                excluded=set(),
                reporter=report)
        except RecursionError:
            result = f'recursed out (adjacency_list={graph.adjacency_list})'
        else:
            seconds = time.process_time() - begin
            current = sorted(sorted(clique) for clique in report.cliques)
            if first is None:
                first = current
            if first == current:
                result = f'OK, {seconds:4.1f}s, {report.cnt} recursive calls'
            else:
                result = f'oops, {first} != {current}'
        print(f'## {func.__name__}@{graph.name}: {result}')


def random_graph(order, size):
    assert size < order * (order - 1) // 2
    vertices = range(1, order + 1)
    unsaturated_vertices = list(vertices)
    a = [None] + [[] for _ in range(order)]
    for _ in range(size):
        v = random.choice(unsaturated_vertices)
        w = random.choice([w for w in vertices if w != v and w not in a[v]])
        assert v not in a[w]
        a[v] += [w]
        a[w] += [v]
        if len(a[v]) == order - 1:
            unsaturated_vertices.remove(v)
        if len(a[w]) == order - 1:
            unsaturated_vertices.remove(w)
    return Graph(name=f'random_of_order_{order}_size_{size}', adjacency_list=a)


if __name__ == '__main__':
    test_on_graph(Graph(name='empty', adjacency_list=[None]))
    test_on_graph(Graph(name='single', adjacency_list=[None, []]))
    test_on_graph(Graph(name='sample', adjacency_list=SAMPLE_ADJACENCY_LIST))

    size_by_order = {
        10: [0, 20],
        100: [0, 200],
        1000: [0, 2000],
        10000: [0, 1000, 2000, 4000, 8000, 16000, 32000],
    }
    for order, sizes in size_by_order.items():
        for size in sizes:
            test_on_graph(random_graph(order=order, size=size))
