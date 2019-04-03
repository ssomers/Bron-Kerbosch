# coding: utf-8

from graph import UndirectedGraph


class PriorityQueue:
    def __init__(self, max_priority):
        self.stack_per_priority = [[] for _ in range(max_priority + 1)]

    def put(self, priority, element):
        assert priority >= 0
        self.stack_per_priority[priority].append(element)

    def pop(self):
        for stack in self.stack_per_priority:
            try:
                return stack.pop()
            except IndexError:
                pass


def degeneracy_order(graph: UndirectedGraph):
    priority_per_node = [-2] * graph.order
    max_degree = 0
    num_candidates = 0
    for c in range(graph.order):
        degree = graph.degree(c)
        if degree > 0:
            priority_per_node[c] = degree
            max_degree = max(max_degree, degree)
            num_candidates += 1
    # Possible values of priority_per_node:
    #   -2: if unconnected (should never come up again)
    #   -1: when yielded
    #   0..max_degree: candidates still queued with priority (degree - #of yielded neighbours)
    q = PriorityQueue(max_priority=max_degree)
    for c, p in enumerate(priority_per_node):
        if p > 0:
            q.put(priority=p, element=c)

    for _ in range(num_candidates):
        i = q.pop()
        while priority_per_node[i] == -1:
            # was requeued with a more urgent priority and therefore already picked
            i = q.pop()
        assert priority_per_node[i] >= 0
        priority_per_node[i] = -1
        yield i
        for v in graph.adjacencies[i]:
            p = priority_per_node[v]
            if p != -1:
                assert p > 0
                # Requeue with a more urgent priority, but don't bother to remove
                # the original entry - it will be skipped if it's reached at all.
                priority_per_node[v] = p - 1
                q.put(priority=p - 1, element=v)
