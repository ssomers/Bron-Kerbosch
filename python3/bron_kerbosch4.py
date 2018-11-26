# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
import random
from typing import List, Set


def bron_kerbosch4(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)

    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = random.sample(candidates | excluded, k=1)[0]
    """
    pivot = random.sample(
        random.choices(
            population=[candidates, excluded],
            weights=[len(candidates), len(excluded)],
            k=1)[0],
        k=1)[0]
    """
    for v in list(candidates.difference(graph.adjacencies[pivot])):
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
