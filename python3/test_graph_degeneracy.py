# coding: utf-8

from graph import Vertex, UndirectedGraph
from graph_degeneracy import degeneracy_ordering

from hypothesis import given, settings
from hypothesis.strategies import builds, integers, lists, sets
from typing import List, Set


def is_like_set(lst: List, st: Set) -> bool:
    return len(lst) == len(st) and set(lst) == st


def symmetric_adjacencies(adjac: List[Set[Vertex]]):
    order = len(adjac)
    adjacencies: List[Set[Vertex]] = [set() for _ in range(order)]
    for v, neighbours in enumerate(adjac):
        for w in neighbours:
            if v != w:
                adjacencies[v].add(w)
                adjacencies[w].add(v)
    return adjacencies


def are_unique(lst: List[int]):
    return len(lst) == len(set(lst))


def test_degeneracy_ordering_empty():
    g = UndirectedGraph(adjacencies=[])
    assert list(degeneracy_ordering(g)) == []
    assert list(degeneracy_ordering(g, drop=1)) == []

@given(builds(
    symmetric_adjacencies,
    integers(min_value=1, max_value=99).flatmap(lambda order:
        lists(sets(integers(min_value=0, max_value=order-1)),
              min_size=order, max_size=order)
    )))
def test_degeneracy_ordering_nonempty(adjacencies):
    g = UndirectedGraph(adjacencies=adjacencies)
    connected_vertices = g.connected_vertices()
    ordering = list(degeneracy_ordering(g))
    ordering_min1 = list(degeneracy_ordering(g, drop=1))
    assert are_unique(ordering)
    assert are_unique(ordering_min1)
    assert set(ordering) == connected_vertices
    assert set(ordering_min1) <= connected_vertices
    assert len(ordering_min1) == max(len(connected_vertices), 1) - 1
