# coding: utf-8

from bron_kerbosch_pivot import pick_max_degree, visit
from graph import UndirectedGraph, Vertex
from reporter import Reporter
from typing import List, Set


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


def degeneracy_order_smart(graph: UndirectedGraph, candidates: Set[Vertex]):
    order = graph.order
    infinite = order * 2  # still >= order after decrementing in each iteration
    degree_per_node = [infinite] * order
    max_degree = 0
    for node in candidates:
        degree = graph.degree(node)
        assert degree > 0  # FYI, isolated nodes were excluded up front
        max_degree = max(degree, max_degree)
        degree_per_node[node] = degree
    nodes_per_degree: List[List[Vertex]] = [
        [] for degree in range(max_degree + 1)
    ]
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
