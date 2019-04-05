# coding: utf-8

from bron_kerbosch_pivot import pick_arbitrary, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with arbitrarily chosen pivot'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            initial_pivot_choice=pick_arbitrary,
            further_pivot_choice=pick_arbitrary,
            candidates=candidates,
            excluded=set(),
            clique=[])
