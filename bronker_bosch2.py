# coding: utf-8

MIN_SIZE = 3


def bron_kerbosch2(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron–Kerbosch algorithm with pivot'''
    reporter.inc_count()
    if not candidates and not excluded:
        if len(clique) >= MIN_SIZE:
            reporter.record(clique)
        return

    pivot = pick_random(candidates or excluded)
    for v in list(candidates.difference(NEIGHBORS[pivot])):
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch2(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        candidates.remove(v)
        excluded.add(v)


def pick_random(s):
    assert s
    elem = s.pop()
    s.add(elem)
    return elem
