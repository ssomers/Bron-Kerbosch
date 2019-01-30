# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch1(graph: UndirectedGraph, reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm'''
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
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=clique + [v])
        excluded.add(v)
