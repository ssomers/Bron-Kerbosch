# coding: utf-8

from graph import Vertex, UndirectedGraph
from graph_degeneracy import degeneracy_ordering

from hypothesis import given
from hypothesis.strategies import builds, integers, lists, sets
from typing import Any, List, Set


def is_like_set(lst: List[Any], st: Set[Any]) -> bool:
    return len(lst) == len(st) and set(lst) == st


def symmetric_adjacencies(adjac: List[Set[Vertex]]) -> List[Set[Vertex]]:
    order = len(adjac)
    adjacencies: List[Set[Vertex]] = [set() for _ in range(order)]
    for v, neighbours in enumerate(adjac):
        for w in neighbours:
            if v != w:
                adjacencies[v].add(w)
                adjacencies[w].add(v)
    return adjacencies


def test_degeneracy_ordering_empty() -> None:
    g = UndirectedGraph(adjacencies=[])
    assert list(degeneracy_ordering(g)) == []
    assert list(degeneracy_ordering(g, drop=1)) == []


@given(
    builds(
        symmetric_adjacencies,
        integers(min_value=1,
                 max_value=99).flatmap(lambda order: lists(sets(
                     integers(min_value=0, max_value=order - 1)),
                                                           min_size=order,
                                                           max_size=order))))
def test_degeneracy_ordering_nonempty(adjacencies: List[Set[Vertex]]) -> None:
    g = UndirectedGraph(adjacencies=adjacencies)
    connected_vertices = g.connected_vertices()

    ordering = list(degeneracy_ordering(g))
    assert set(ordering) == connected_vertices
    assert all(g.degree(ordering[0]) <= g.degree(v) for v in ordering[1:])

    ordering_min1 = list(degeneracy_ordering(g, drop=1))
    assert ordering_min1 == ordering[:-1]
