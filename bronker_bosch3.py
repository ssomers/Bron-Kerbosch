# coding: utf-8

from bronker_bosch2 import bron_kerbosch2


def bron_kerbosch3(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    recursing into bron_kerbosch2'''
    reporter.inc_count()
    if not candidates and not excluded:
        assert len(clique) == 0
        return

    for v in list(degeneracy_order(NEIGHBORS, candidates)):
        assert NEIGHBORS[v]
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch2(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        candidates.remove(v)
        excluded.add(v)


def bron_kerbosch6(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron-Kerbosch algorithm with pivot and degeneracy ordering,
    recursing into itself'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    for v in degeneracy_order(NEIGHBORS, candidates):
        assert NEIGHBORS[v]
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch6(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        candidates.remove(v)
        excluded.add(v)


def degeneracy_order(NEIGHBORS, nodes):
    # FIXME: can improve it to linear time
    deg = {node: len(NEIGHBORS[node]) for node in nodes}

    while deg:
        i = min(deg, key=deg.get)
        yield i
        del deg[i]
        for v in NEIGHBORS[i]:
            if v in deg:
                deg[v] -= 1
