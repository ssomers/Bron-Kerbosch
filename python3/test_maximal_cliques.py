# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2
from bronker_bosch3 import bron_kerbosch3
from bronker_bosch4 import bron_kerbosch4
from bronker_bosch5 import bron_kerbosch5
from bronker_bosch6 import bron_kerbosch6
from data import NEIGHBORS as SAMPLE_ADJACENCY_LIST
from graph import UndirectedGraph as Graph
from graph import random_undirected_graph
from reporter import SimpleReporter
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
        seed = random.randrange(1 << 32)
    if args.order is not None and args.size is not None:
        sizes_by_order = {int(args.order): [int(size) for size in args.size]}
    else:
        assert False, "Run with -O for meaningful measurements"
        sizes_by_order = {
            50:
            list(range(750, 1000, 10)),  # max 1225
            10_000:
            list(range(1_000, 10_000, 1_000)) + list(
                range(10_000, 100_000, 10_000)),
        }
    for order, sizes in sizes_by_order.items():
        times_per_size = []
        for size in sizes:
            random.seed(seed)
            g = random_graph(order=order, size=size)
            times_per_size.append(bron_kerbosch_timed(g))

        try:
            from plotly import graph_objs, plotly
        except ImportError as e:
            print(f"{e}, not plotting until you pip install plotly")
        else:
            traces = [
                graph_objs.Scatter(
                    x=sizes,
                    y=[times_per_size[s][f] for s in range(len(sizes))],
                    mode='lines+markers',
                    name=f"Ver{f+1}") for f in range(len(funcs))
            ]
            layout = {
                'title': ("Implementations of Bron-Kerbosch on " +
                          f"random graphs order (#nodes) {order}"),
                'xaxis': {
                    'title': "Size (#edges)"
                },
                'yaxis': {
                    'title': "Seconds spent"
                },
            }
            plotly.plot(
                figure_or_data={
                    'data': traces,
                    'layout': layout,
                },
                filename=f'Bron-Kerbosch_order_{order}')
    print(f"random seed was {seed}")
