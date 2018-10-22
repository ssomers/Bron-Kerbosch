from typing import List, Optional, Set


class Graph(object):
    def __init__(self, adjacencies: List[Set[int]],
                 name: Optional[str] = None):
        for v, adjacent_to_v in enumerate(adjacencies):
            if v == 0:
                assert adjacent_to_v is None, "start at index 1"
            else:
                for w in adjacent_to_v:
                    assert w != v
                    assert v in adjacencies[w], (
                        f'{w} is adjacent to {v} but not vice versa')
        self.adjacencies = adjacencies
        self.name = name

    @property
    def order(self):
        return len(self.adjacencies) - 1

    @property
    def nodes(self):
        return range(1, len(self.adjacencies))

    def connected_nodes(self):
        return {n for n in self.nodes if self.adjacencies[n]}
