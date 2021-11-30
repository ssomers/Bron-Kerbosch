from typing import List, Set

Vertex = int


class UndirectedGraph(object):
    def __init__(self, adjacencies: List[Set[Vertex]]):
        order = len(adjacencies)
        for v, adjacent_to_v in enumerate(adjacencies):
            for w in adjacent_to_v:
                assert 0 <= w < order
                assert v != w
                assert v in adjacencies[w], f'{w} is adjacent to {v} but not vice versa'
        self.adjacencies = adjacencies

    @property
    def order(self) -> int:
        return len(self.adjacencies)

    def size(self) -> int:
        total = sum(len(a) for a in self.adjacencies)
        assert total % 2 == 0
        return total // 2

    def degree(self, node: Vertex) -> int:
        return len(self.adjacencies[node])

    def connected_vertices(self) -> Set[Vertex]:
        return {node for node in range(self.order) if self.adjacencies[node]}
