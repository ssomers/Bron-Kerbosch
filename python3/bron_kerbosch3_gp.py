# coding: utf-8

import bron_kerbosch_degen
from graph import UndirectedGraph
from consumer import CliqueConsumer


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """
    Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
    choosing a pivot from candidates only (IK_GP)
    """
    bron_kerbosch_degen.explore(graph, consumer, pivot_choice_X=False)
