# coding: utf-8

from random_graph import random_undirected_graph
import random


def test_random_graph():
    random.seed(19680516)
    random_undirected_graph(order=2, size=0)
    random_undirected_graph(order=3, size=0)
    random_undirected_graph(order=3, size=1)
    random_undirected_graph(order=3, size=2)
    random_undirected_graph(order=4, size=0)
    random_undirected_graph(order=4, size=1)
    random_undirected_graph(order=4, size=2)
    random_undirected_graph(order=4, size=3)
    random_undirected_graph(order=4, size=4)
    random_undirected_graph(order=4, size=5)
