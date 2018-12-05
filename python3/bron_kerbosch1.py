# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
from typing import List, Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm'''
    visit(
        graph=graph,
        reporter=reporter,
        candidates=graph.connected_nodes(),
        excluded=set(),
        clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter,
          candidates: Set[int], excluded: Set[int], clique: List[int]):
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
