# coding: utf-8

from abc import abstractmethod, ABCMeta
from typing import List, Sequence


class Reporter(metaclass=ABCMeta):
    @abstractmethod
    def record(self, clique: Sequence[int]) -> None:
        pass


class CollectingReporter(Reporter):
    def __init__(self) -> None:
        self.cliques: List[Sequence[int]] = []

    def record(self, clique: Sequence[int]) -> None:
        assert len(clique) > 1
        self.cliques.append(clique)


class CountingReporter(Reporter):
    def __init__(self) -> None:
        self.cliques = 0

    def record(self, clique: Sequence[int]) -> None:
        self.cliques += 1
