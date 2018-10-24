# coding: utf-8


def bron_kerbosch1(NEIGHBORS, clique, candidates, excluded, reporter):
    '''Naive Bron-Kerbosch algorithm'''
    reporter.inc_count()
    if not candidates and not excluded:
        reporter.record(clique)

    while candidates:
        v = candidates.pop()
        assert NEIGHBORS[v]
        new_candidates = candidates.intersection(NEIGHBORS[v])
        new_excluded = excluded.intersection(NEIGHBORS[v])
        bron_kerbosch1(NEIGHBORS, clique + [v], new_candidates, new_excluded,
                       reporter)
        excluded.add(v)
