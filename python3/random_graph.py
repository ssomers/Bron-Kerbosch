from graph import UndirectedGraph, Vertex
import argparse
import os
import random
import sys
from typing import Generator, List, Optional, Set, Tuple


class NeighbourhoodWatch:

    def __init__(self, order: int):
        vertices = range(order)
        self.unsaturated_vertices = list(vertices)
        self.neighbours: List[Set[int]] = [set() for _ in vertices]
        self.complement: List[bool] = [False] * order

    def is_saturated(self) -> bool:
        return len(self.unsaturated_vertices) < 2

    def pick_pair(self) -> Tuple[int, int]:
        v = random.choice(self.unsaturated_vertices)
        if self.complement[v]:
            assert self.neighbours[v]
            w = random.choice(tuple(self.neighbours[v]))
        else:
            w = v
            while w == v or w in self.neighbours[v]:
                w = random.choice(self.unsaturated_vertices)
        assert v != w
        return v, w

    def meet_neighbour(self, v: int, new_neighbour: int) -> None:
        assert v != new_neighbour
        if self.complement[v]:
            assert new_neighbour in self.neighbours[v]
            self.neighbours[v].remove(new_neighbour)
            if not self.neighbours[v]:
                self.unsaturated_vertices.remove(v)
        else:
            assert new_neighbour not in self.neighbours[v]
            n = len(self.neighbours[v])
            if n + 1 >= len(self.unsaturated_vertices) - 2 - n:
                self.complement[v] = True
                self.neighbours[v] = (set(self.unsaturated_vertices) - {v} -
                                      {new_neighbour} - self.neighbours[v])
                if not self.neighbours[v]:
                    self.unsaturated_vertices.remove(v)
            else:
                self.neighbours[v].add(new_neighbour)


def generate_edges(order: int) -> Generator[Tuple[int, int], None, None]:
    nw = NeighbourhoodWatch(order)
    while not nw.is_saturated():
        v, w = nw.pick_pair()
        nw.meet_neighbour(v, w)
        nw.meet_neighbour(w, v)
        yield (min(v, w), max(v, w))


def generate_n_edges(
        order: int,
        size: Optional[int] = None) -> Generator[Tuple[int, int], None, None]:
    fully_meshed_size = order * (order - 1) // 2
    if size is None:
        size = fully_meshed_size
    elif size > fully_meshed_size:
        raise ValueError(
            f"{order} nodes accommodate at most {fully_meshed_size} edges")
    for (v, w), _ in zip(generate_edges(order), range(size)):
        yield v, w


def random_undirected_graph(order: int, size: int) -> UndirectedGraph:
    adjacencies: List[Set[Vertex]] = [set() for _ in range(order)]
    print(f"order={order}, size={size}")
    for v, w in generate_n_edges(order, size):
        adjacencies[v].add(w)
        adjacencies[w].add(v)
    print(adjacencies)
    g = UndirectedGraph(adjacencies=adjacencies)
    assert g.order == order
    assert g.size() == size
    return g


def read_random_graph(orderstr: str,
                      size: Optional[int]) -> Tuple[UndirectedGraph, int]:
    order = to_int(orderstr)
    fully_meshed_size = order * (order - 1) // 2
    if size is None:
        size = fully_meshed_size
    elif size > fully_meshed_size:
        raise ValueError(
            f"{order} nodes accommodate at most {fully_meshed_size} edges")
    edges_name = "random_edges_order_" + orderstr
    stats_name = "random_stats"
    edges_path = os.path.join(os.pardir, "data", edges_name + ".txt")
    stats_path = os.path.join(os.pardir, "data", stats_name + ".txt")
    adjacencies = read_edges(edges_path, orderstr, order, size)
    clique_count = read_stats(stats_path, orderstr, size)
    g = UndirectedGraph(adjacencies=adjacencies)
    assert g.order == order
    assert g.size() == size
    return g, clique_count


def read_edges(path: str, orderstr: str, order: int,
               size: int) -> List[Set[Vertex]]:
    adjacencies: List[Set[Vertex]] = [set() for _ in range(order)]
    try:
        with open(path, 'r') as txtfile:
            for i, line in enumerate(txtfile):
                strs = line.split()
                try:
                    v = int(strs[0])
                    w = int(strs[1])
                except (IndexError, ValueError):
                    raise ValueError(
                        f"File {path} has bogus line “{line.rstrip()}”"
                    ) from None
                adjacencies[v].add(w)
                adjacencies[w].add(v)
                if i + 1 == size:
                    return adjacencies
            else:
                raise ValueError(
                    f"Exhausted generated list of {i+1} edges in {path}")
    except OSError as err:
        raise ValueError(
            f"{err}\nPerhaps generate it with"
            f" `python -m random_graph {orderstr} <max_size?>`"
        ) from err


def read_stats(path: str, orderstr: str, size: int) -> int:
    try:
        prefix = f"{orderstr}\t{size}\t"
        with open(path, 'r') as txtfile:
            for line in txtfile:
                if line.startswith(prefix):
                    try:
                        return int(line[len(prefix):])
                    except (ValueError):
                        raise ValueError(
                            f"File {path} has bogus line “{line.rstrip()}”"
                        ) from None
    except OSError as err:
        raise ValueError() from err
    raise ValueError(f"File {path} lacks order {orderstr} size {size}")


def to_int(txt: str) -> int:
    if txt is None:
        return None
    elif txt.endswith('M'):
        return int(txt[:-1]) * 1_000_000
    elif txt.endswith('k'):
        return int(txt[:-1]) * 1_000
    else:
        return int(txt)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="generate edges for undirected graph")
    parser.add_argument('order', nargs=1)
    parser.add_argument('size', nargs='?')
    args = parser.parse_args(sys.argv[1:])
    orderstr = args.order[0]
    sizestr = args.size
    order = to_int(orderstr)
    size = to_int(sizestr)
    filename = f"random_edges_order_{orderstr}"
    path = os.path.join(os.pardir, "data", filename + ".txt")
    print(f"Writing {size or 'all'} edges to {path}")
    with open(path, 'w', newline='\n') as txtfile:
        for (v, w) in generate_n_edges(order=order, size=size):
            txtfile.write(f"{v} {w}\n")
