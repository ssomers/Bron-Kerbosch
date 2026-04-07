# coding: utf-8

from graph import UndirectedGraph, Vertex
from typing import Generator, List


class Degeneracy:
    def __init__(self, graph: UndirectedGraph) -> None:
        self.graph = graph
        # Possible values of priorityPerVertex (after initialization):
        #   0: never queued because not connected (degree 0),
        #      or no longer queued because it has been yielded itself,
        #      or no longer queued because all neighbours have been yielded
        #   1 or more: candidates queued with priority (degree - #of yielded neighbours)
        self.priority_per_node = [0] * graph.order
        self.q = PriorityQueue(max_priority=graph.max_degree)
        self.num_left_to_pick = 0
        for v in range(graph.order):
            if priority := graph.degree(v):
                self.priority_per_node[v] = priority
                self.q.put(priority=priority, element=v)
                self.num_left_to_pick += 1

    def is_candidate(self, v: Vertex) -> bool:
        return self.priority_per_node[v] != 0

    def iter(self) -> Generator[Vertex, None, None]:
        """
        Iterate connected vertices, lowest degree first, skipping vertices whose neighbours
        have all been yielded.
        """
        while self.num_left_to_pick > 0:
            pick = self.q.pop()
            if self.priority_per_node[pick] > 0:
                self.priority_per_node[pick] = 0
                neighbours = self.graph.adjacencies[pick]
                self.num_left_to_pick -= 1
                yield pick
                for v in neighbours:
                    if (old_priority := self.priority_per_node[v]) > 0:
                        # Requeue with a more urgent priority or dequeue.
                        # Don't bother to remove the original entry from the queue,
                        # since the vertex will be skipped when popped, and thanks to
                        # num_left_to_pick we might not need to pop it at all.
                        new_priority = old_priority - 1
                        self.priority_per_node[v] = new_priority
                        if new_priority > 0:
                            self.q.put(priority=new_priority, element=v)
                        else:
                            self.num_left_to_pick -= 1
                assert self.num_left_to_pick >= 0


class PriorityQueue:
    def __init__(self, max_priority: int) -> None:
        self.stack_per_priority: List[List[int]] = [[] for _ in range(max_priority)]

    def put(self, priority: int, element: int) -> None:
        assert priority > 0
        self.stack_per_priority[priority - 1].append(element)

    def pop(self) -> int:
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass
        else:
            raise ValueError("attempt to pop more than was put")
