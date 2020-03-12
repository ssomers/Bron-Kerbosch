# coding: utf-8

from abc import abstractmethod, ABCMeta


class Reporter(metaclass=ABCMeta):
    @abstractmethod
    def record(self, clique):
        pass


class SimpleReporter(Reporter):
    def __init__(self):
        self.cliques = []

    def record(self, clique):
        assert len(clique) > 1
        self.cliques.append(clique)


class CountingReporter(Reporter):
    def __init__(self):
        self.cliques = 0

    def record(self, clique):
        self.cliques += 1
