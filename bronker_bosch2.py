# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
from typing import List, Set


def bron_kerbosch2(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = pick_random(candidates or excluded)
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


def pick_random(s):
    assert s
    elem = s.pop()
    s.add(elem)
    return elem


def bron_kerbosch4(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = next(iter(candidates or excluded))
    assert graph.adjacencies[pivot]
    for v in candidates - graph.adjacencies[pivot]:
        neighbours = graph.adjacencies[v]
        assert neighbours
        candidates.remove(v)
        bron_kerbosch4(
            graph=graph,
            clique=clique + [v],
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            reporter=reporter)
        excluded.add(v)
