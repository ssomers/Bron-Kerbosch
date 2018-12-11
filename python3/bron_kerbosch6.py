# coding: utf-8

from graph import UndirectedGraph
from reporter import Reporter
import itertools
from typing import List, Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates,
            excluded=set(),
            clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[int],
          excluded: Set[int], clique: List[int]):
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)

    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = pick_best(graph=graph, candidates=candidates, excluded=excluded)
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


def pick_best(graph: UndirectedGraph, candidates: Set[int],
              excluded: Set[int]) -> int:
    assert candidates or excluded

    max_degree = -1
    best = None
    for node in itertools.chain(candidates, excluded):
        degree = len(graph.adjacencies[node] & candidates)
        if max_degree < degree:
            max_degree = degree
            best = node
    assert best is not None
    return best
