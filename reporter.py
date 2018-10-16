# coding: utf-8


class Reporter(object):
    def __init__(self, name):
        self.name = name
        self.cnt = 0
        self.cliques = []

    def inc_count(self):
        self.cnt += 1

    def record(self, clique):
        self.cliques.append(clique)

    def print_report(self):
        print(self.name)
        print(f'{self.cnt} recursive calls')
        for i, clique in enumerate(self.cliques):
            print(f'{i}: {clique}')
        print
