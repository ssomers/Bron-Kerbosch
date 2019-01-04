# coding: utf-8

import bron_kerbosch2
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering'''
    reporter.inc_count()
    candidates = graph.connected_nodes()
    excluded: Set[Vertex] = set()
    for v in list(degeneracy_order(graph=graph, nodes=candidates)):
        neighbours = graph.adjacencies[v]
        assert neighbours
        bron_kerbosch2.visit(
            graph=graph,
            reporter=reporter,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=[v])
        candidates.remove(v)
        excluded.add(v)


def degeneracy_order(graph: UndirectedGraph, nodes: Set[Vertex]):
    # FIXME: can improve it to linear time
    deg = {node: graph.degree(node) for node in nodes}

    while deg:
        i = min(deg, key=deg.get)
        yield i
        del deg[i]
        for v in graph.adjacencies[i]:
            if v in deg:
                deg[v] -= 1
