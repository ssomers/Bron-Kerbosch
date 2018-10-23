from typing import List, Optional, Set


class UndirectedGraph(object):
    def __init__(self, adjacencies: List[Set[int]],
                 name: Optional[str] = None):
        for v, adjacent_to_v in enumerate(adjacencies):
            for w in adjacent_to_v:
                assert v != w
                assert v in adjacencies[w], (
                    f'{w} is adjacent to {v} but not vice versa')
        self.adjacencies = adjacencies
        self.name = name

    @property
    def order(self) -> int:
        return len(self.adjacencies)

    def size(self) -> int:
        total = sum(len(a) for a in self.adjacencies)
        assert total % 2 == 0
        return total // 2

    def connected_nodes(self) -> Set[int]:
        return {n for n in range(self.order) if self.adjacencies[n]}
