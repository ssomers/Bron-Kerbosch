# coding: utf-8

from bronker_bosch5 import bron_kerbosch5
from graph import UndirectedGraph
from reporter import Reporter
from dataclasses import dataclass, field
import queue
from typing import Any, List, Set


def bron_kerbosch6(graph: UndirectedGraph, clique: List[int],
                   candidates: Set[int], excluded: Set[int],
                   reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    optimized'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    for v in degeneracy_order_smart(graph=graph, candidates=candidates):
        neighbours = graph.adjacencies[v]
        assert neighbours
        candidates.remove(v)
        bron_kerbosch5(
            graph=graph,
            clique=clique + [v],
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            reporter=reporter)
        excluded.add(v)


@dataclass(order=True)
class PrioritizedItem:
    priority: int
    node: int = field(compare=False)


# poorly performing alternative to degeneracy_order_smart
def degeneracy_order_queue(graph: UndirectedGraph, candidates: Set[int]):
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


def pick_with_lowest_degree(degree_per_node, nodes_per_degree, infinite):
    assert all(node in nodes_per_degree[degree]
               for node, degree in enumerate(degree_per_node)
               if degree != infinite)
    for d in range(len(nodes_per_degree)):
        while nodes_per_degree[d]:
            v = nodes_per_degree[d].pop()
            if degree_per_node[v] != infinite:
                return v
            else:
                continue  # was moved to lower degree


def degeneracy_order_smart(graph: UndirectedGraph, candidates: Set[int]):
    if not candidates:
        return
    order = graph.order
    infinite = order * 2  # still >= order after decrementing in each iteration
    degree_per_node = [infinite] * order
    max_degree = 0
    for node in candidates:
        degree = graph.degree(node)
        assert degree > 0  # FYI, isolated nodes were excluded up front
        max_degree = max(degree, max_degree)
        degree_per_node[node] = degree
    nodes_per_degree: List[List[int]] = [[]
                                         for degree in range(max_degree + 1)]
    for node in candidates:
        degree = graph.degree(node)
        nodes_per_degree[degree].append(node)

    for _ in range(len(candidates)):
        i = pick_with_lowest_degree(
            degree_per_node=degree_per_node,
            nodes_per_degree=nodes_per_degree,
            infinite=infinite)
        degree_per_node[i] = infinite
        yield i
        for v in graph.adjacencies[i]:
            d = degree_per_node[v]
            if d != infinite:
                degree_per_node[v] = d - 1
                # move to lower degree, but no need to remove the original one
                nodes_per_degree[d - 1].append(v)
