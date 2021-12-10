# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter

from typing import Callable, List, Set
import random

PivotChoice = Callable[[UndirectedGraph, Set[Vertex]], Vertex]


def visit(graph: UndirectedGraph, reporter: Reporter, pivot_choice_X: bool,
          candidates: Set[Vertex], excluded: Set[Vertex],
          clique: List[Vertex]) -> None:
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)
    assert candidates.isdisjoint(excluded)
    assert len(candidates) >= 1
    if len(candidates) == 1:
        # Same logic as below, stripped down for this common case
        for v in candidates:
            neighbours = graph.adjacencies[v]
            assert neighbours
            if excluded.isdisjoint(neighbours):
                reporter.record(clique + [v])
        return

    # Quickly handle locally unconnected candidates while finding pivot
    remaining_candidates = []
    seen_local_degree = 0
    for v in candidates:
        neighbours = graph.adjacencies[v]
        local_degree = len(candidates.intersection(neighbours))
        if local_degree == 0:
            # Same logic as below, stripped down
            if neighbours.isdisjoint(excluded):
                reporter.record(clique + [v])
        else:
            if seen_local_degree < local_degree:
                seen_local_degree = local_degree
                pivot = v
            remaining_candidates.append(v)
    if seen_local_degree == 0:
        return
    if pivot_choice_X:
        for v in excluded:
            neighbours = graph.adjacencies[v]
            local_degree = len(candidates.intersection(neighbours))
            if seen_local_degree < local_degree:
                seen_local_degree = local_degree
                pivot = v

    for v in remaining_candidates:
        neighbours = graph.adjacencies[v]
        assert neighbours
        if pivot not in neighbours:
            candidates.remove(v)
            if neighbouring_candidates := candidates.intersection(neighbours):
                neighbouring_excluded = excluded.intersection(neighbours)
                visit(graph=graph,
                      reporter=reporter,
                      pivot_choice_X=pivot_choice_X,
                      candidates=neighbouring_candidates,
                      excluded=neighbouring_excluded,
                      clique=clique + [v])
            elif excluded.isdisjoint(neighbours):
                reporter.record(clique + [v])
            excluded.add(v)
