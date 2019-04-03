# coding: utf-8

from bron_kerbosch_degeneracy import degeneracy_order
from bron_kerbosch_pivot import pick_max_degree_local, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import Set


def bron_kerbosch3_gpx(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    optimized'''
    candidates = graph.connected_nodes()
    excluded: Set[Vertex] = set()
    assert len(candidates) == len(list(degeneracy_order(graph=graph)))
    assert candidates == set(degeneracy_order(graph=graph))
    for v in degeneracy_order(graph=graph):
        neighbours = graph.adjacencies[v]
        assert neighbours
        candidates.remove(v)
        visit(
            graph=graph,
            reporter=reporter,
            initial_pivot_choice=pick_max_degree_local,
            further_pivot_choice=pick_max_degree_local,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=[v])
        excluded.add(v)
