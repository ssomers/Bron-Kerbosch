from typing import List, Set
import random

Vertex = int


class UndirectedGraph(object):
    def __init__(self, adjacencies: List[Set[Vertex]]):
        for v, adjacent_to_v in enumerate(adjacencies):
            for w in adjacent_to_v:
                assert v != w
                assert v in adjacencies[w], (
                    f'{w} is adjacent to {v} but not vice versa')
        self.adjacencies = adjacencies

    @property
    def order(self) -> int:
        return len(self.adjacencies)

    def size(self) -> int:
        total = sum(len(a) for a in self.adjacencies)
        assert total % 2 == 0
        return total // 2

    def degree(self, node):
        return len(self.adjacencies[node])

    def connected_vertices(self) -> Set[Vertex]:
        return {node for node in range(self.order) if self.adjacencies[node]}


def random_undirected_graph(order: int, size: int) -> UndirectedGraph:
    fully_meshed_size = order * (order - 1) // 2
    if size > fully_meshed_size:
        raise ValueError(
            f"{order} nodes accommodate at most {fully_meshed_size} edges")
    vertices = range(order)
    unsaturated_vertices = list(vertices)
    adjacency_sets: List[Set[Vertex]] = [set() for _ in range(order)]
    adjacency_complements: List[Set[Vertex]] = [set()] * order
    for _ in range(size):
        v = random.choice(unsaturated_vertices)
        assert len(adjacency_sets[v]) < order - 1
        if adjacency_complements[v]:
            w = random.sample(adjacency_complements[v], 1)[0]
        else:
            w = v
            while w == v or w in adjacency_sets[v]:
                w = random.choice(unsaturated_vertices)
        assert v != w
        assert w not in adjacency_sets[v]
        assert v not in adjacency_sets[w]
        for x, y in [(v, w), (w, v)]:
            adjacency_sets[x].add(y)
            neighbours = len(adjacency_sets[x])
            if neighbours == order - 1:
                unsaturated_vertices.remove(x)
            elif neighbours == order // 2:
                # start using adjacency complement
                assert not adjacency_complements[x]
                adjacency_complements[x] = (
                    set(unsaturated_vertices) - {x} - adjacency_sets[x])
            elif neighbours > order // 2:
                adjacency_complements[x].remove(y)
    g = UndirectedGraph(adjacencies=adjacency_sets)
    assert g.order == order
    assert g.size() == size
    return g
