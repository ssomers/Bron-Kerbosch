# coding: utf-8

from bron_kerbosch_degeneracy import degeneracy_order
from bron_kerbosch_pivot import pick_max_degree_local, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import Set


def bron_kerbosch3_gpx(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with degeneracy ordering,
    recursing with pivot of highest degree towards the remaining candidates (IK_GPX)'''
    candidates = graph.connected_nodes()
    excluded: Set[Vertex] = set()
    assert len(candidates) == len(list(degeneracy_order(graph=graph)))
    assert candidates == set(degeneracy_order(graph=graph))
    for v in degeneracy_order(graph=graph):
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
                initial_pivot_choice=pick_max_degree_local,
                further_pivot_choice=pick_max_degree_local,
                candidates=neighbouring_candidates,
                excluded=neighbouring_excluded,
                clique=[v])
        excluded.add(v)
