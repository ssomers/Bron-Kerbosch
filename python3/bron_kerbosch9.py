# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from dataclasses import dataclass, field
import queue
from typing import Any, List, Set


def explore(graph: UndirectedGraph, reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    optimized'''
    reporter.inc_count()
    candidates = graph.connected_nodes()
    excluded: Set[Vertex] = set()
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
    degree_by_node = {c: graph.degree(c) for c in candidates}
    q: Any = queue.PriorityQueue()
    for c in candidates:
        q.put(PrioritizedItem(priority=graph.degree(c), node=c))

    while not q.empty():
        i = q.get().node
        try:
            del degree_by_node[i]
        except KeyError:
            pass  # was moved to lower degree
        else:
            yield i
            for v in graph.adjacencies[i]:
                p = degree_by_node.get(v)
                if p is not None:
                    degree_by_node[v] = p - 1
                    q.put(PrioritizedItem(priority=p - 1, node=v))
