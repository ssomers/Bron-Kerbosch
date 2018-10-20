class Graph(object):
    def __init__(self, name, adjacency_list):
        self.name = name
        self.adjacency_list = adjacency_list
        self.validate()

    def validate(self):
        assert self.adjacency_list[0] is None
        for v, neighbours in enumerate(self.adjacency_list):
            if v == 0:
                assert neighbours is None  # start from index 1
                continue
            for n in neighbours:
                assert n != v
                assert v in self.adjacency_list[
                    n], f'{v} not in neighbours of {n}'

    @property
    def order(self):
        return len(self.adjacency_list) - 1

    @property
    def nodes(self):
        return range(1, len(self.adjacency_list))
