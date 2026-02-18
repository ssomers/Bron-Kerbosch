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


def degeneracy_filter(graph: UndirectedGraph) -> Generator[Vertex, None, None]:
    """
    Iterate connected vertices, lowest degree first, skipping vertices whose neighbours
    have all been yielded.
    """
    priority_per_node = [0] * graph.order
    max_degree = 0
    for c in range(graph.order):
        if degree := graph.degree(c):
            priority_per_node[c] = degree
            max_degree = max(max_degree, degree)
    # Possible values of priority_per_node:
    #   -2: if unconnected (should never be queried)
    #   -1: after having been yielded
    #   0..max_degree: candidates still queued with priority equal to
    #                  (degree - number of yielded neighbours).
    q = PriorityQueue(max_priority=max_degree)
    num_left_to_pick = 0
    for c, p in enumerate(priority_per_node):
        if p > 0:
            q.put(priority=p, element=c)
            num_left_to_pick += 1

    while num_left_to_pick > 0:
        pick = q.pop()
        if priority_per_node[pick] > 0:
            # In contrast to most languages, python allows spawning as soon as possible,
            # before we adjust the data. Not that we know it makes a difference.
            yield pick
            priority_per_node[pick] = 0
            num_left_to_pick -= 1
            for v in graph.adjacencies[pick]:
                if (old_priority := priority_per_node[v]) > 0:
                    # Requeue with a more urgent priority or dequeue.
                    # Don't bother to remove the original entry from the queue,
                    # since the vertex will be skipped when popped, and thanks to
                    # num_left_to_pick we might not need to pop it at all.
                    new_priority = old_priority - 1
                    priority_per_node[v] = new_priority
                    if new_priority > 0:
                        q.put(priority=new_priority, element=v)
                    else:
                        num_left_to_pick -= 1
