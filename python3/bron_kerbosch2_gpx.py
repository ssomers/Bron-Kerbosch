# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, pick_max_degree_local, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2_gpx(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)'''
    candidates = graph.connected_nodes()
    if candidates:
        visit(
            graph=graph,
            reporter=reporter,
            initial_pivot_choice=pick_max_degree,
            further_pivot_choice=pick_max_degree_local,
            candidates=candidates,
            excluded=set(),
            clique=[])
