# coding: utf-8

from bron_kerbosch_pivot import visit
from graph import UndirectedGraph, Vertex
from consumer import CliqueConsumer
from typing import Set


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """
    Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
    chosen from candidates only (IK_GP)
    """
    if graph.order:
        pivot = max(range(graph.order), key=graph.degree)
        excluded: Set[Vertex] = set()
        for v in range(graph.order):
            neighbours = graph.adjacencies[v]
            if neighbours and pivot not in neighbours:
                neighbouring_excluded = excluded.intersection(neighbours)
                if len(neighbouring_excluded) < len(neighbours):
                    neighbouring_candidates = neighbours.difference(
                        neighbouring_excluded
                    )
                    visit(
                        graph=graph,
                        consumer=consumer,
                        pivot_choice_X=False,
                        candidates=neighbouring_candidates,
                        excluded=neighbouring_excluded,
                        clique=[v],
                    )
                else:
                    assert not excluded.isdisjoint(neighbours)
                excluded.add(v)
