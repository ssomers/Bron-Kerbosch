# coding: utf-8

from graph import UndirectedGraph, Vertex
from reporter import Reporter

from enum import Enum
from typing import Callable, List, Set
import itertools
import random

PivotChoice = Callable


def visit(graph: UndirectedGraph, reporter: Reporter,
          initial_pivot_choice: PivotChoice, further_pivot_choice: PivotChoice,
          candidates: Set[Vertex], excluded: Set[Vertex],
          clique: List[Vertex]):
    assert candidates
    assert all(graph.degree(v) > 0 for v in candidates)
    assert all(graph.degree(v) > 0 for v in excluded)

    if len(candidates) == 1:
        # Same logic as below, stripped down for this common case
        for v in candidates:
            neighbours = graph.adjacencies[v]
            assert neighbours
            if excluded.isdisjoint(neighbours):
                reporter.record(clique + [v])
        return

    if initial_pivot_choice in [pick_max_degree_local, pick_max_degree_localX]:
        # Quickly handle locally unconnected candidates while finding pivot
        remaining_candidates = []
        seen_local_degree = 0
        for v in candidates:
            neighbours = graph.adjacencies[v]
            local_degree = len(neighbours.intersection(candidates))
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
        if initial_pivot_choice == pick_max_degree_localX:
            for v in excluded:
                neighbours = graph.adjacencies[v]
                local_degree = len(neighbours.intersection(candidates))
                if seen_local_degree < local_degree:
                    seen_local_degree = local_degree
                    pivot = v
    else:
        pivot = initial_pivot_choice(graph=graph, candidates=candidates)
        remaining_candidates = list(candidates)

    for v in remaining_candidates:
        neighbours = graph.adjacencies[v]
        assert neighbours
        if pivot in neighbours:
            continue
        candidates.remove(v)
        neighbouring_candidates = candidates.intersection(neighbours)
        if neighbouring_candidates:
            neighbouring_excluded = excluded.intersection(neighbours)
            visit(
                graph=graph,
                reporter=reporter,
                initial_pivot_choice=further_pivot_choice,
                further_pivot_choice=further_pivot_choice,
                candidates=neighbouring_candidates,
                excluded=neighbouring_excluded,
                clique=clique + [v])
        else:
            if excluded.isdisjoint(neighbours):
                reporter.record(clique + [v])
        excluded.add(v)


def pick_arbitrary(graph: UndirectedGraph, candidates: Set[Vertex]) -> Vertex:
    return next(iter(candidates))


def pick_random(graph: UndirectedGraph, candidates: Set[Vertex]) -> Vertex:
    return random.sample(candidates, k=1)[0]
    """
    return random.sample(
        random.choices(
            population=[candidates, excluded],
            weights=[len(candidates), len(excluded)],
            k=1)[0],
        k=1)[0]
    """


def pick_max_degree(graph: UndirectedGraph, candidates: Set[Vertex]) -> Vertex:
    return max(candidates, key=graph.degree)


def pick_max_degree_local():
    pass


def pick_max_degree_localX():
    pass
