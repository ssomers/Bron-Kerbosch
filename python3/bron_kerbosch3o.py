# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


def bron_kerbosch3o(graph: UndirectedGraph, reporter: Reporter):
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


class PriorityQueue:
    def __init__(self, max_priority):
        self.stack_per_priority = [[] for _ in range(max_priority + 1)]

    def put(self, priority, element):
        assert priority >= 0
        self.stack_per_priority[priority].append(element)

    def pop(self):
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass


def degeneracy_order_smart(graph: UndirectedGraph, candidates: Set[Vertex]):
    priority_per_node = [-2] * graph.order
    max_degree = 0
    for c in candidates:
        degree = graph.degree(c)
        assert degree > 0  # only connected nodes are candidates
        priority_per_node[c] = degree
        max_degree = max(max_degree, degree)
    # Possible values of priority_per_node:
    #   -2: if unconnected (should never come up again)
    #   -1: when yielded
    #   0..max_degree: candidates still queued with priority (degree - #of yielded neighbours)
    q = PriorityQueue(max_priority=max_degree)
    for c, p in enumerate(priority_per_node):
        if p > 0:
            q.put(priority=p, element=c)

    for _ in range(len(candidates)):
        i = q.pop()
        while priority_per_node[i] == -1:
            # was requeued with a more urgent priority and therefore already picked
            i = q.pop()
        assert priority_per_node[i] >= 0
        priority_per_node[i] = -1
        yield i
        for v in graph.adjacencies[i]:
            p = priority_per_node[v]
            if p != -1:
                assert p > 0
                # Requeue with a more urgent priority, but don't bother to remove
                # the original entry - it will be skipped if it's reached at all.
                priority_per_node[v] = p - 1
                q.put(priority=p - 1, element=v)
