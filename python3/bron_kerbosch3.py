# coding: utf-8

from bron_kerbosch_pivot import pick_arbitrary, visit
from graph import UndirectedGraph, Vertex
from graph_degeneracy import degeneracy_ordering
from reporter import Reporter
from typing import Set


def bron_kerbosch3(graph: UndirectedGraph, reporter: Reporter) -> None:
    '''Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
       choosing a pivot arbitrarily'''
    excluded: Set[Vertex] = set()
    for v in degeneracy_ordering(graph=graph, drop=1):
        neighbours = graph.adjacencies[v]
        assert neighbours
        if neighbouring_candidates := neighbours.difference(excluded):
            neighbouring_excluded = neighbours.intersection(excluded)
            visit(graph=graph,
                  reporter=reporter,
                  initial_pivot_choice=pick_arbitrary,
                  further_pivot_choice=pick_arbitrary,
                  candidates=neighbouring_candidates,
                  excluded=neighbouring_excluded,
                  clique=[v])
        else:
            assert not excluded.isdisjoint(neighbours)
        excluded.add(v)
