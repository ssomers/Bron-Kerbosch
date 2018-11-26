# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
from typing import List, Set


def bron_kerbosch1(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        bron_kerbosch1(
            graph=graph,
            clique=clique + [v],
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            reporter=reporter)
        excluded.add(v)
