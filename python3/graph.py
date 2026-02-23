from typing import List, Set, TypeAlias

Vertex: TypeAlias = int


class UndirectedGraph(object):
    def __init__(self, adjacencies: List[Set[Vertex]]):
        order = len(adjacencies)
        for v, adjacent_to_v in enumerate(adjacencies):
            for w in adjacent_to_v:
                assert 0 <= w < order
                assert v != w, f"{v} is adjacent to itself"
                assert v in adjacencies[w], f"{w} is adjacent to {v} but not vice versa"
        self.adjacencies = adjacencies
        self.max_degree = max(len(a) for a in self.adjacencies) if order else 0
        total_degree = sum(len(a) for a in self.adjacencies)
        assert total_degree % 2 == 0
        self.size = total_degree / 2

    @property
    def order(self) -> int:
        return len(self.adjacencies)

    def degree(self, node: Vertex) -> int:
        return len(self.adjacencies[node])

    def has_degree(self, node: Vertex) -> bool:
        return bool(self.adjacencies[node])

    def connected_vertices(self) -> Set[Vertex]:
        return {node for node in range(self.order) if self.has_degree(node)}
