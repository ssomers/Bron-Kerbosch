# coding: utf-8

from abc import abstractmethod, ABCMeta
from typing import List, Sequence


class CliqueConsumer(metaclass=ABCMeta):
    @abstractmethod
    def accept(self, clique: Sequence[int]) -> None:
        pass


class CliqueCollector(CliqueConsumer):
    def __init__(self) -> None:
        self.cliques: List[Sequence[int]] = []

    def accept(self, clique: Sequence[int]) -> None:
        assert len(clique) > 1
        self.cliques.append(clique)


class CliqueCounter(CliqueConsumer):
    def __init__(self) -> None:
        self.cliques = 0

    def accept(self, clique: Sequence[int]) -> None:
        self.cliques += 1
