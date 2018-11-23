# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
from typing import List, Set


def bron_kerbosch2(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot'''
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)

    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = pick_arbitrary(candidates or excluded)
    assert graph.adjacencies[pivot]
    for v in list(candidates.difference(graph.adjacencies[pivot])):
        assert graph.adjacencies[v]
        bron_kerbosch2(
            graph=graph,
            clique=clique + [v],
            candidates=candidates.intersection(graph.adjacencies[v]),
            excluded=excluded.intersection(graph.adjacencies[v]),
            reporter=reporter)
        candidates.remove(v)
        excluded.add(v)


def pick_arbitrary(s):
    # same as next(iter(s)), but that doesn't seem to affect performance
    assert s
    elem = s.pop()
    s.add(elem)
    return elem
