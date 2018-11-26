# coding: utf-8

from bron_kerbosch2 import bron_kerbosch2
from graph import UndirectedGraph
from reporter import Reporter
from typing import List, Set


def bron_kerbosch3(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering'''
    reporter.inc_count()
    if not candidates and not excluded:
        assert len(clique) == 0
        return

    for v in list(degeneracy_order(graph=graph, nodes=candidates)):
        assert graph.adjacencies[v]
        bron_kerbosch2(
            graph=graph,
            clique=clique + [v],
            candidates=candidates.intersection(graph.adjacencies[v]),
            excluded=excluded.intersection(graph.adjacencies[v]),
            reporter=reporter)
        candidates.remove(v)
        excluded.add(v)


def degeneracy_order(graph: UndirectedGraph, nodes: Set[int]):
    # FIXME: can improve it to linear time
    deg = {node: graph.degree(node) for node in nodes}

    while deg:
        i = min(deg, key=deg.get)
        yield i
        del deg[i]
        for v in graph.adjacencies[i]:
            if v in deg:
                deg[v] -= 1
