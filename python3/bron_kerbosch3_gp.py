# coding: utf-8

from bron_kerbosch_pivot import visit
from graph import UndirectedGraph, Vertex
from graph_degeneracy import degeneracy_ordering
from reporter import Reporter
from typing import Set


def explore(graph: UndirectedGraph, reporter: Reporter) -> None:
    """
    Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
    choosing a pivot from candidates only (IK_GP)
    """
    # In this initial iteration, we don't need to represent the set of candidates
    # because all neighbours are candidates until excluded.
    excluded: Set[Vertex] = set()
    for v in degeneracy_ordering(graph=graph, drop=1):
        neighbours = graph.adjacencies[v]
        assert neighbours
        neighbouring_excluded = neighbours.intersection(excluded)
        if len(neighbouring_excluded) < len(neighbours):
            neighbouring_candidates = neighbours.difference(neighbouring_excluded)
            visit(
                graph=graph,
                reporter=reporter,
                pivot_choice_X=False,
                candidates=neighbouring_candidates,
                excluded=neighbouring_excluded,
                clique=[v],
            )
        else:
            assert not excluded.isdisjoint(neighbours)
        excluded.add(v)
