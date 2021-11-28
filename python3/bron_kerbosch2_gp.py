# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, pick_max_degree_local, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2_gp(graph: UndirectedGraph, reporter: Reporter) -> None:
    '''Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
       chosen from candidates only (IK_GP)'''
    if candidates := graph.connected_vertices():
        visit(graph=graph,
              reporter=reporter,
              initial_pivot_choice=pick_max_degree,
              further_pivot_choice=pick_max_degree_local,
              candidates=candidates,
              excluded=set(),
              clique=[])
