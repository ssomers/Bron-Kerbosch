# coding: utf-8

from bron_kerbosch_pivot import visit
from graph import UndirectedGraph, Vertex
from graph_degeneracy import degeneracy_filter
from consumer import CliqueConsumer
from typing import Set


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """
    Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
    choosing a pivot from both candidates and excluded vertices (IK_GPX)
    """
    # In this initial iteration, we don't need to represent the set of candidates
    # because all neighbours are candidates until excluded.
    excluded: Set[Vertex] = set()
    for v in degeneracy_filter(graph=graph):
        neighbours = graph.adjacencies[v]
        assert neighbours
        neighbouring_excluded = neighbours.intersection(excluded)
        neighbouring_candidates = neighbours.difference(neighbouring_excluded)
        visit(
            graph=graph,
            consumer=consumer,
            pivot_choice_X=True,
            candidates=neighbouring_candidates,
            excluded=neighbouring_excluded,
            clique=[v],
        )
        excluded.add(v)
