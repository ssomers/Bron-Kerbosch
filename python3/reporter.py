# coding: utf-8

from abc import abstractmethod, ABCMeta


class Reporter(metaclass=ABCMeta):
    @abstractmethod
    def inc_count(self):
        pass

    @abstractmethod
    def record(self, clique):
        pass


class SimpleReporter(Reporter):
    def __init__(self):
        self.cnt = 0
        self.cliques = []

    def inc_count(self):
        self.cnt += 1

    def record(self, clique):
        if len(clique) > 1:
            self.cliques.append(clique)