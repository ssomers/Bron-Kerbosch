# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch1o(graph: UndirectedGraph, reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm optimized'''
    if candidates := graph.connected_vertices():
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates,
            excluded=set(),
            clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[Vertex],
          excluded: Set[Vertex], clique: List[Vertex]):
    assert candidates

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        neighbouring_candidates = candidates.intersection(neighbours)
        if neighbouring_candidates:
            visit(
                graph=graph,
                reporter=reporter,
                candidates=neighbouring_candidates,
                excluded=excluded.intersection(neighbours),
                clique=clique + [v])
        else:
            if excluded.isdisjoint(neighbours):
                reporter.record(clique + [v])
        excluded.add(v)
