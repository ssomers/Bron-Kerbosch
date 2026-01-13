# coding: utf-8

from graph import UndirectedGraph, Vertex
from consumer import CliqueConsumer
from typing import List, Set


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """Naive Bron-Kerbosch algorithm"""
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
    if not candidates and not excluded:
        consumer.accept(clique)

    while candidates:
        v = candidates.pop()
        neighbours = graph.adjacencies[v]
        assert neighbours
        visit(
            graph=graph,
            consumer=consumer,
            candidates=candidates.intersection(neighbours),
            excluded=excluded.intersection(neighbours),
            clique=clique + [v],
        )
        excluded.add(v)
