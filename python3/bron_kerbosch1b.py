# coding: utf-8

from graph import UndirectedGraph, Vertex
from consumer import CliqueConsumer
from typing import List, Set


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """Naive Bron-Kerbosch algorithm, optimized"""
    if candidates := graph.connected_vertices():
        visit(
            graph=graph,
            consumer=consumer,
            candidates=candidates,
            excluded=set(),
            clique=[],
        )


def visit(
    graph: UndirectedGraph,
    consumer: CliqueConsumer,
    candidates: Set[Vertex],
    excluded: Set[Vertex],
    clique: List[Vertex],
) -> None:
    assert all(graph.has_degree(v) for v in candidates)
    assert all(graph.has_degree(v) for v in excluded)
    assert candidates.isdisjoint(excluded)
    assert candidates
    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        neighbouring_candidates = candidates.intersection(neighbours)
        if neighbouring_candidates:
            neighbouring_excluded = excluded.intersection(neighbours)
            visit(
                graph,
                consumer,
                candidates=neighbouring_candidates,
                excluded=neighbouring_excluded,
                clique=clique + [v],
            )
        elif excluded.isdisjoint(neighbours):
            consumer.accept(clique + [v])
        excluded.add(v)
