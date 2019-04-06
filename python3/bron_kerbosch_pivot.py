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

    pivot = initial_pivot_choice(
        graph=graph, candidates=candidates, excluded=excluded)
    for v in list(candidates.difference(graph.adjacencies[pivot])):
        neighbours = graph.adjacencies[v]
        assert neighbours
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


def pick_arbitrary(graph: UndirectedGraph, candidates: Set[Vertex],
                   excluded: Set[Vertex]) -> Vertex:
    return next(iter(candidates))


def pick_random(graph: UndirectedGraph, candidates: Set[Vertex],
                excluded: Set[Vertex]) -> Vertex:
    return random.sample(candidates, k=1)[0]
    """
    return random.sample(
        random.choices(
            population=[candidates, excluded],
            weights=[len(candidates), len(excluded)],
            k=1)[0],
        k=1)[0]
    """


def pick_max_degree(graph: UndirectedGraph, candidates: Set[Vertex],
                    excluded: Set[Vertex]) -> Vertex:
    return max(candidates, key=lambda v: graph.degree(v))


def pick_max_degree_local(graph: UndirectedGraph, candidates: Set[Vertex],
                          excluded: Set[Vertex]) -> Vertex:
    return max(
        itertools.chain(candidates, excluded),
        key=lambda v: len(graph.adjacencies[v] & candidates))
