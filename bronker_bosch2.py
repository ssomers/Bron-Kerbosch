# coding: utf-8


def bron_kerbosch2(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron-Kerbosch algorithm with pivot'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = pick_random(candidates or excluded)
    assert NEIGHBORS[pivot]
    for v in list(candidates.difference(NEIGHBORS[pivot])):
        assert NEIGHBORS[v]
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


def bron_kerbosch4(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Bron-Kerbosch algorithm with pivot, slightly optimized'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)
        return

    pivot = next(iter(candidates or excluded))
    assert NEIGHBORS[pivot]
    for v in candidates.difference(NEIGHBORS[pivot]):
        assert NEIGHBORS[v]
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch4(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        candidates.remove(v)
        excluded.add(v)
