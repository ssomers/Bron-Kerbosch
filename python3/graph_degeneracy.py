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
    queue = PriorityQueue(max_priority=graph.max_degree)
    for v in range(graph.order):
        priority = graph.degree(v)
        priority_per_node[v] = priority
        queue.insert(v, priority)

    while not queue.empty():
        pick = queue.pop()
        if priority_per_node[pick] > 0:
            priority_per_node[pick] = 0
            # In contrast to most languages, python allows spawning as soon as possible,
            # before we adjust the data. Not that we know it makes a difference.
            yield pick
            queue.forget(pick)
            for v in graph.adjacencies[pick]:
                if (old_priority := priority_per_node[v]) > 0:
                    new_priority = old_priority - 1
                    priority_per_node[v] = new_priority
                    queue.promote(v, new_priority)


class PriorityQueue:
    def __init__(self, max_priority: int) -> None:
        self.stack_per_priority: List[List[int]] = [[] for _ in range(max_priority)]
        self.num_left_to_pick = 0

    def empty(self) -> bool:
        return self.num_left_to_pick == 0

    def insert(self, element: int, priority: int) -> None:
        if priority > 0:
            self.stack_per_priority[priority - 1].append(element)
            self.num_left_to_pick += 1

    # Requeue with a more urgent priority or dequeue.
    # Don't bother to remove the original entry from the queue,
    # since the vertex will be skipped when popped, and thanks to
    # num_left_to_pick we might not need to pop it at all.
    def promote(self, element: int, priority: int) -> None:
        if priority > 0:
            self.stack_per_priority[priority - 1].append(element)
        else:
            self.forget(element)

    def forget(self, element: int) -> None:
        assert self.num_left_to_pick > 0
        self.num_left_to_pick -= 1

    # We may return an element already popped, even though it was passed to forget,
    # in case its priority was promoted earlier on. That's why we do not count
    # the element as picked, but wait for the caller to forget it. The caller must
    # somehow ensure to forget the same element only once.
    def pop(self) -> int:
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass
        else:
            raise ValueError("attempt to pop more than was put")
