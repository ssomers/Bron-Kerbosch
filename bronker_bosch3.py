# coding: utf-8

from bronker_bosch2 import bron_kerbosch2
from reporter import Reporter
from dataclasses import dataclass, field
import queue
from typing import List, Set


def bron_kerbosch3(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    recursing into bron_kerbosch2'''
    reporter.inc_count()
    if not candidates and not excluded:
        assert len(clique) == 0
        return

    for v in list(degeneracy_order(NEIGHBORS, candidates)):
        assert NEIGHBORS[v]
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch2(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        candidates.remove(v)
        excluded.add(v)


def degeneracy_order(NEIGHBORS, nodes):
    # FIXME: can improve it to linear time
    deg = {node: len(NEIGHBORS[node]) for node in nodes}

    while deg:
        i = min(deg, key=deg.get)
        yield i
        del deg[i]
        for v in NEIGHBORS[i]:
            if v in deg:
                deg[v] -= 1


def bron_kerbosch6(NEIGHBORS, clique: List[int], candidates: Set[int],
                   excluded: Set[int], reporter: Reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    recursing into itself'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    for v in degeneracy_order_newer(NEIGHBORS, candidates):
        assert NEIGHBORS[v]
        candidates.remove(v)
        bron_kerbosch6(
            NEIGHBORS=NEIGHBORS,
            clique=clique + [v],
            candidates=candidates.intersection(NEIGHBORS[v]),
            excluded=excluded.intersection(NEIGHBORS[v]),
            reporter=reporter)
        excluded.add(v)


def degeneracy_order_array(NEIGHBORS, candidates):
    order = len(NEIGHBORS)
    infinite = order * 2  # still >= order after decrementing in each iteration
    degree_per_node = [infinite] * order
    for node in candidates:
        degree_per_node[node] = len(NEIGHBORS[node])

    for _ in range(len(candidates)):
        i = min(zip(degree_per_node, range(order)))[1]
        yield i
        degree_per_node[i] = infinite
        for v in NEIGHBORS[i]:
            degree_per_node[v] -= 1


@dataclass(order=True)
class PrioritizedItem:
    priority: int
    node: int = field(compare=False)


def degeneracy_order_queue(NEIGHBORS, nodes):
    degree_by_node = {node: len(NEIGHBORS[node]) for node in nodes}
    q = queue.PriorityQueue()
    for node in nodes:
        q.put(PrioritizedItem(priority=len(NEIGHBORS[node]), node=node))

    while not q.empty():
        i = q.get().node
        try:
            del degree_by_node[i]
        except KeyError:
            pass  # was moved to lower degree
        else:
            yield i
            for v in NEIGHBORS[i]:
                p = degree_by_node.get(v)
                if p is not None:
                    degree_by_node[v] = p - 1
                    q.put(PrioritizedItem(priority=p - 1, node=v))


def pick_with_lowest_degree(degree_by_node, nodes_per_degree):
    assert all(node in nodes_per_degree[degree]
               for node, degree in degree_by_node.items())
    for d in range(len(nodes_per_degree)):
        while nodes_per_degree[d]:
            v = nodes_per_degree[d].pop()
            if v in degree_by_node:
                return v
            else:
                continue  # was moved to lower degree


def pick_with_lowest_degree_newer(degree_per_node, nodes_per_degree, infinite):
    assert all(node in nodes_per_degree[degree]
               for node, degree in enumerate(degree_per_node)
               if degree != infinite)
    d = 0
    while True:
        while nodes_per_degree[d]:
            v = nodes_per_degree[d].pop()
            if degree_per_node[v] != infinite:
                return v
            else:
                continue  # was moved to lower degree
        d += 1


def degeneracy_order_new(NEIGHBORS, candidates):
    if not candidates:
        return
    degree_by_node = {c: len(NEIGHBORS[c]) for c in candidates}
    max_degree = max(degree_by_node.values())
    nodes_per_degree = [[] for degree in range(max_degree + 1)]
    for c, degree in degree_by_node.items():
        assert degree > 0  # FYI, isolated nodes were excluded up front
        nodes_per_degree[degree].append(c)

    for _ in range(len(candidates)):
        i = pick_with_lowest_degree(degree_by_node, nodes_per_degree)
        yield i
        del degree_by_node[i]
        for v in NEIGHBORS[i]:
            d = degree_by_node.get(v)
            if d is not None:
                degree_by_node[v] = d - 1
                # move to lower degree, but no need to remove the original one
                nodes_per_degree[d - 1].append(v)


def degeneracy_order_newer(NEIGHBORS, candidates):
    if not candidates:
        return
    order = len(NEIGHBORS)
    infinite = order * 2  # still >= order after decrementing in each iteration
    # degree_by_node = {node: len(NEIGHBORS[node]) for node in candidates}
    degree_per_node = [infinite] * order
    max_degree = 0
    for node in candidates:
        degree = len(NEIGHBORS[node])
        assert degree > 0  # FYI, isolated nodes were excluded up front
        max_degree = max(degree, max_degree)
        degree_per_node[node] = degree
    nodes_per_degree = [[] for degree in range(max_degree + 1)]
    for node in candidates:
        degree = len(NEIGHBORS[node])
        nodes_per_degree[degree].append(node)

    for _ in range(len(candidates)):
        i = pick_with_lowest_degree_newer(
            degree_per_node=degree_per_node,
            nodes_per_degree=nodes_per_degree,
            infinite=infinite)
        degree_per_node[i] = infinite
        yield i
        for v in NEIGHBORS[i]:
            d = degree_per_node[v]
            if d != infinite:
                degree_per_node[v] = d - 1
                # move to lower degree, but no need to remove the original one
                nodes_per_degree[d - 1].append(v)
