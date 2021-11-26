# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch0(graph: UndirectedGraph, reporter: Reporter):
    '''Naive Bron-Kerbosch algorithm'''
    if candidates := graph.connected_vertices():
        visit(graph=graph,
              reporter=reporter,
              candidates=candidates,
              excluded=set(),
              clique=[])


def visit(graph: UndirectedGraph, reporter: Reporter, candidates: Set[Vertex],
          excluded: Set[Vertex], clique: List[Vertex]):
    if not candidates and not excluded:
        reporter.record(clique)

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        visit(graph=graph,
              reporter=reporter,
              candidates=candidates.intersection(neighbours),
              excluded=excluded.intersection(neighbours),
              clique=clique + [v])
        excluded.add(v)
