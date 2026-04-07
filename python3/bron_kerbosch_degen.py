# coding: utf-8

from bron_kerbosch_pivot import visit
from graph import UndirectedGraph, Vertex
from graph_degeneracy import Degeneracy
from consumer import CliqueConsumer
from typing import Set


def explore(
    graph: UndirectedGraph,
    consumer: CliqueConsumer,
    pivot_choice_X: bool,
) -> None:
    degeneracy = Degeneracy(graph=graph)
    for v in degeneracy.iter():
        neighbours = graph.adjacencies[v]
        neighbouring_candidates: Set[Vertex] = set()
        neighbouring_excluded: Set[Vertex] = set()
        for w in neighbours:
            if degeneracy.is_candidate(w):
                neighbouring_candidates.add(w)
            else:
                neighbouring_excluded.add(w)
        visit(
            graph=graph,
            consumer=consumer,
            pivot_choice_X=pivot_choice_X,
            candidates=neighbouring_candidates,
            excluded=neighbouring_excluded,
            clique=[v],
        )
