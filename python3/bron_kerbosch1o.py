# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import Any, List, Optional, Set, Tuple


def bron_kerbosch1o(graph: UndirectedGraph, reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm optimized'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates,
            excluded=set(),
            clique=None)


VertexStack = Optional[Tuple[Any, Vertex]]


def append_to(lst: List[Vertex], clique: VertexStack):
    if clique is not None:
        append_to(lst, clique[0])
        lst.append(clique[1])


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[Vertex],
          excluded: Set[Vertex], clique: VertexStack):
    reporter.inc_count()
    if not candidates and not excluded:
        lst: List[Vertex] = []
        append_to(lst, clique)
        reporter.record(lst)

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=(clique, v))
        excluded.add(v)