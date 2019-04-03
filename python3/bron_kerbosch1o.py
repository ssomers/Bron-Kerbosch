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


def length(clique: VertexStack) -> int:
    return 0 if clique is None else 1 + length(clique[0])


def append_to(lst: List[Vertex], at: int, clique: VertexStack) -> int:
    if clique is None:
        return at
    else:
        at = append_to(lst, at, clique[0])
        lst[at] = clique[1]
        return at + 1


def collect(clique: VertexStack):
    lst: List[Vertex] = [-1] * length(clique)
    append_to(lst, 0, clique)
    return lst


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[Vertex],
          excluded: Set[Vertex], clique: VertexStack):
    if not candidates and not excluded:
        reporter.record(collect(clique))

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
