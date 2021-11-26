# coding: utf-8

from bron_kerbosch_pivot import pick_random, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2_rp(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot chosen randomly (IK_RP)'''
    if candidates := graph.connected_vertices():
        visit(graph=graph,
              reporter=reporter,
              initial_pivot_choice=pick_random,
              further_pivot_choice=pick_random,
              candidates=candidates,
              excluded=set(),
              clique=[])
