# coding: utf-8

from bron_kerbosch_pivot import pick_arbitrary, visit
from graph import UndirectedGraph
from reporter import Reporter


def bron_kerbosch2(graph: UndirectedGraph, reporter: Reporter) -> None:
    """Bron-Kerbosch algorithm with arbitrarily chosen pivot"""
    if candidates := graph.connected_vertices():
        visit(graph=graph,
              reporter=reporter,
              initial_pivot_choice=pick_arbitrary,
              further_pivot_choice=pick_arbitrary,
              candidates=candidates,
              excluded=set(),
              clique=[])
