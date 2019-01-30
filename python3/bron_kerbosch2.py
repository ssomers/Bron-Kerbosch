# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch2(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with arbitrarily chosen pivot'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates,
            excluded=set(),
            clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[Vertex],
          excluded: Set[Vertex], clique: List[Vertex]):
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)

    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = pick_arbitrary(candidates or excluded)
    for v in list(candidates.difference(graph.adjacencies[pivot])):
        assert graph.adjacencies[v]
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates.intersection(graph.adjacencies[v]),
            excluded=excluded.intersection(graph.adjacencies[v]),
            clique=clique + [v])
        candidates.remove(v)
        excluded.add(v)


def pick_arbitrary(s):
    # same as next(iter(s)), but that doesn't seem to affect performance
    assert s
    elem = s.pop()
    s.add(elem)
    return elem
