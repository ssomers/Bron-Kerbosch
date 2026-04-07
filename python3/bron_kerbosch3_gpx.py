# coding: utf-8

import bron_kerbosch_degen
from graph import UndirectedGraph
from consumer import CliqueConsumer


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """
    Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
    choosing a pivot from both candidates and excluded vertices (IK_GPX)
    """
    bron_kerbosch_degen.explore(graph, consumer, pivot_choice_X=True)
