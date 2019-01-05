# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from dataclasses import dataclass, field
import queue
from typing import List, Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    optimized'''
    reporter.inc_count()
    candidates = graph.connected_nodes()
    excluded: Set[Vertex] = set()
    assert candidates == set(
        degeneracy_order_smart(graph=graph, candidates=candidates))
    for v in degeneracy_order_smart(graph=graph, candidates=candidates):
        neighbours = graph.adjacencies[v]
        assert neighbours
        candidates.remove(v)
        visit(
            graph=graph,
            reporter=reporter,
            initial_pivot_choice=pick_max_degree,
            further_pivot_choice=pick_max_degree,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=[v])
        excluded.add(v)


@dataclass(order=True)
class PrioritizedItem:
    priority: int
    node: Vertex = field(compare=False)


def degeneracy_order_smart(graph: UndirectedGraph, candidates: Set[Vertex]):
    priority_per_node = [-2] * graph.order
    q: queue.PriorityQueue = queue.PriorityQueue()
    for c in candidates:
        p = graph.degree(c)
        priority_per_node[c] = p
        q.put(PrioritizedItem(priority=p, node=c))

    for _ in range(len(candidates)):
        i = q.get(block=False).node
        while priority_per_node[i] == -1:
            i = q.get(block=False).node
        assert priority_per_node[i] >= 0
        priority_per_node[i] = -1
        yield i
        for v in graph.adjacencies[i]:
            p = priority_per_node[v]
            if p != -1:
                assert p > 0
                priority_per_node[v] = p - 1
                q.put(PrioritizedItem(priority=p - 1, node=v), block=False)
