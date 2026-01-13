# coding: utf-8

from bron_kerbosch_pivot import visit
from graph import UndirectedGraph
from consumer import CliqueConsumer


def explore(graph: UndirectedGraph, consumer: CliqueConsumer) -> None:
    """
    Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
    chosen from candidates only (IK_GP)
    """
    if candidates := graph.connected_vertices():
        visit(
            graph=graph,
            consumer=consumer,
            pivot_choice_X=False,
            candidates=candidates,
            excluded=set(),
            clique=[],
        )
