# coding: utf-8

from graph import UndirectedGraph, Vertex
from typing import Generator, List


def degeneracy_filter(graph: UndirectedGraph) -> Generator[Vertex, None, None]:
    """
    Iterate connected vertices, lowest degree first, skipping vertices whose neighbours
    have all been yielded.
    """
    # Possible values of priorityPerVertex (after initialization):
    #   0: never queued because not connected (degree 0),
    #      or no longer queued because it has been yielded itself,
    #      or no longer queued because all neighbours have been yielded
    #   1 or more: candidates queued with priority (degree - #of yielded neighbours)
    priority_per_node = [0] * graph.order
    q = PriorityQueue(max_priority=graph.max_degree)
    num_left_to_pick = 0
    for v in range(graph.order):
        if priority := graph.degree(v):
            priority_per_node[v] = priority
            q.put(priority=priority, element=v)
            num_left_to_pick += 1

    while num_left_to_pick > 0:
        pick = q.pop()
        if priority_per_node[pick] > 0:
            # In contrast to most languages, python allows spawning as soon as possible,
            # before we adjust the data. Not that we know it makes a difference.
            yield pick
            priority_per_node[pick] = 0
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
            num_left_to_pick -= 1
            assert num_left_to_pick >= 0


class PriorityQueue:
    def __init__(self, max_priority: int) -> None:
        self.stack_per_priority: List[List[int]] = [[] for _ in range(max_priority)]

    def put(self, priority: int, element: int) -> None:
        assert priority > 0
        self.stack_per_priority[priority-1].append(element)

    def pop(self) -> int:
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass
        else:
            raise ValueError("attempt to pop more than was put")



