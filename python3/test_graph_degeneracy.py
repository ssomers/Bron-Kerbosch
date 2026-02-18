# coding: utf-8

from graph import Vertex, UndirectedGraph
from graph_degeneracy import degeneracy_filter

from hypothesis import given
from hypothesis.strategies import builds, integers, lists, sets
from typing import List, Set


def symmetric_adjacencies(adjac: List[Set[Vertex]]) -> List[Set[Vertex]]:
    order = len(adjac)
    adjacencies: List[Set[Vertex]] = [set() for _ in range(order)]
    for v, neighbours in enumerate(adjac):
        for w in neighbours:
            if v != w:
                adjacencies[v].add(w)
                adjacencies[w].add(v)
    return adjacencies


def test_degeneracy_filter_empty() -> None:
    g = UndirectedGraph(adjacencies=[])
    assert list(degeneracy_filter(g)) == []


@given(
    builds(
        symmetric_adjacencies,
        integers(min_value=1, max_value=99).flatmap(
            lambda order: lists(
                sets(integers(min_value=0, max_value=order - 1)),
                min_size=order,
                max_size=order,
            )
        ),
    )
)
def test_degeneracy_filter_nonempty(adjacencies: List[Set[Vertex]]) -> None:
    g = UndirectedGraph(adjacencies=adjacencies)
    connected_vertices = g.connected_vertices()

    ordered = list(degeneracy_filter(g))
    assert set(ordered).issubset(connected_vertices)
    assert len(ordered) < max(1, len(connected_vertices))
    assert all(g.degree(ordered[0]) <= g.degree(v) for v in ordered)
