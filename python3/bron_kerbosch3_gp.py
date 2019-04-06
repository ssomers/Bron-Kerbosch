# coding: utf-8

from bron_kerbosch_degeneracy import degeneracy_ordering
from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import Set


def bron_kerbosch3_gp(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with degeneracy ordering,
       recursing with pivot of highest degree (IK_GP)'''
    candidates = graph.connected_vertices()
    excluded: Set[Vertex] = set()
    assert len(candidates) == len(list(degeneracy_ordering(graph=graph)))
    assert candidates == set(degeneracy_ordering(graph=graph))
    for v in degeneracy_ordering(graph=graph):
        neighbours = graph.adjacencies[v]
        assert neighbours
        candidates.remove(v)
        neighbouring_candidates = candidates.intersection(neighbours)
        if neighbouring_candidates:
            neighbouring_excluded = excluded.intersection(neighbours)
            visit(
                graph=graph,
                reporter=reporter,
                initial_pivot_choice=pick_max_degree,
                further_pivot_choice=pick_max_degree,
                candidates=neighbouring_candidates,
                excluded=neighbouring_excluded,
                clique=[v])
        else:
            assert not excluded.isdisjoint(neighbours)
        excluded.add(v)
