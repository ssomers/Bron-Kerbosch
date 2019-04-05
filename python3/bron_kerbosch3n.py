# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch3n(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with degenerate degeneracy ordering'''
    candidates = set()
    max_degree = 0
    for node in range(graph.order):
        degree = graph.degree(node)
        if degree:
            max_degree = max(max_degree, degree)
    candidates_per_degree: List[List[Vertex]] = [
        [] for degree in range(max_degree)
    ]
    for node in range(graph.order):
        degree = graph.degree(node)
        if degree:
            candidates.add(node)
            candidates_per_degree[degree - 1].append(node)

    excluded: Set[Vertex] = set()
    for cands in candidates_per_degree:
        for v in cands:
            neighbours = graph.adjacencies[v]
            assert neighbours
            candidates.remove(v)
            neighbouring_candidates = candidates.intersection(neighbours)
            if not neighbouring_candidates:
                assert not excluded.isdisjoint(neighbours)
            else:
                neighbouring_excluded = excluded.intersection(neighbours)
                visit(
                    graph=graph,
                    reporter=reporter,
                    initial_pivot_choice=pick_max_degree,
                    further_pivot_choice=pick_max_degree,
                    candidates=neighbouring_candidates,
                    excluded=neighbouring_excluded,
                    clique=[v])
            excluded.add(v)
