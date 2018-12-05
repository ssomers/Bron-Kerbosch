# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
import random
from typing import List, Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    visit(
        graph=graph,
        reporter=reporter,
        candidates=graph.connected_nodes(),
        excluded=set(),
        clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter,
          candidates: Set[int], excluded: Set[int], clique: List[int]):
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
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=clique + [v])
        excluded.add(v)
