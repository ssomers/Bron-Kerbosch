# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, pick_max_degree_localX, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2_gpx(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
       chosen from both candidates and excluded vertices (IK_GPX)'''
    if candidates := graph.connected_vertices():
        visit(graph=graph,
              reporter=reporter,
              initial_pivot_choice=pick_max_degree,
              further_pivot_choice=pick_max_degree_localX,
              candidates=candidates,
              excluded=set(),
              clique=[])
