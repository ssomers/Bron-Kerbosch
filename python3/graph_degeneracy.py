# coding: utf-8

from graph import UndirectedGraph, Vertex
from typing import Generator, List


class PriorityQueue:
    def __init__(self, max_priority: int) -> None:
        self.stack_per_priority: List[List[int]] = [[] for _ in range(max_priority + 1)]

    def put(self, priority: int, element: int) -> None:
        assert priority >= 0
        self.stack_per_priority[priority].append(element)

    def pop(self) -> int:
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass
        else:
            raise ValueError("attempt to pop more than was put")


def degeneracy_ordering(
    graph: UndirectedGraph, drop: int = 0
) -> Generator[Vertex, None, None]:
    """
    Iterate connected vertices, lowest degree first.
    drop=N: omit last N vertices
    """
    assert drop >= 0
    priority_per_node = [-2] * graph.order
    max_degree = 0
    num_candidates = 0
    for c in range(graph.order):
        if degree := graph.degree(c):
            priority_per_node[c] = degree
            max_degree = max(max_degree, degree)
            num_candidates += 1
    # Possible values of priority_per_node:
    #   -2: if unconnected (should never be queried)
    #   -1: after having been yielded
    #   0..max_degree: candidates still queued with priority equal to
    #                  (degree - number of yielded neighbours).
    q = PriorityQueue(max_priority=max_degree)
    for c, p in enumerate(priority_per_node):
        if p > 0:
            q.put(priority=p, element=c)

    for _ in range(num_candidates - drop):
        i = q.pop()
        while priority_per_node[i] == -1:
            # was requeued with a more urgent priority and therefore already picked
            i = q.pop()
        assert priority_per_node[i] >= 0
        priority_per_node[i] = -1
        yield i
        for v in graph.adjacencies[i]:
            if (p := priority_per_node[v]) != -1:
                assert p > 0
                # Requeue with a more urgent priority, but don't bother to remove
                # the original entry - it will be skipped if it's reached at all.
                priority_per_node[v] = p - 1
                q.put(priority=p - 1, element=v)
