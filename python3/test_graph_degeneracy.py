# coding: utf-8

from graph import UndirectedGraph
from graph_degeneracy import degeneracy_ordering

from hypothesis import given, settings
from hypothesis.strategies import integers, lists
from typing import List, Set


def is_like_set(lst: List, st: Set) -> bool:
    return len(lst) == len(st) and set(lst) == st


MAX_ORDER = 100


@given(
    lists(
        lists(
            integers(min_value=0, max_value=MAX_ORDER - 1),
            min_size=0,
            max_size=MAX_ORDER),
        min_size=0,
        max_size=MAX_ORDER))
@settings(max_examples=333)
def test_more(given):
    order = len(given)
    adjacencies = [set() for _ in range(order)]
    for v, neighbours in enumerate(given):
        for w in neighbours:
            if w < order and v != w:
                adjacencies[v].add(w)
                adjacencies[w].add(v)
    g = UndirectedGraph(adjacencies)

    ordering = list(degeneracy_ordering(g))
    ordering1 = list(degeneracy_ordering(g, drop=1))
    assert len(ordering) == len(set(ordering))
    assert len(ordering1) == len(set(ordering1))
    assert len(ordering1) == max(len(ordering) - 1, 0)
    assert set(ordering) == g.connected_vertices()
    assert set(ordering1).issubset(g.connected_vertices())
