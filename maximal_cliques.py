# coding: utf-8

from bronker_bosch1 import bron_kerbosch1
from bronker_bosch2 import bron_kerbosch2
from bronker_bosch3 import bron_kerbosch3
from data import EXAMPLE_NEIGHBORS
from reporter import Reporter
import copy

if __name__ == '__main__':
    funcs = [bron_kerbosch1, bron_kerbosch2, bron_kerbosch3]
    for example_name, NEIGHBORS in EXAMPLE_NEIGHBORS.items():
        assert NEIGHBORS[0] == []
        NODES = set(range(1, len(NEIGHBORS)))
        for func in funcs:
            report = Reporter(f'## {func.__name__}@{example_name}')
            func(
                NEIGHBORS=NEIGHBORS,
                clique=[],
                candidates=copy.copy(NODES),
                excluded=set(),
                reporter=report)
            report.print_report()
