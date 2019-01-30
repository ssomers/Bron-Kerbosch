# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2_gp(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            initial_pivot_choice=pick_max_degree,
            further_pivot_choice=pick_max_degree,
            candidates=candidates,
            excluded=set(),
            clique=[])
