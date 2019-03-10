extern crate core;
use std::collections::BTreeSet;

pub enum MyIntersection<'a, T> {
    Stitch {
        a_iter: std::collections::btree_set::Iter<'a, T>,
        b_iter: std::collections::btree_set::Iter<'a, T>,
    },
    Spring {
        a_range: std::collections::btree_set::Range<'a, T>,
        a_set: &'a BTreeSet<T>,
        b_range: std::collections::btree_set::Range<'a, T>,
        b_set: &'a BTreeSet<T>,
    },
    Search {
        a_iter: std::collections::btree_set::Iter<'a, T>,
        b_set: &'a BTreeSet<T>,
    },
}

impl<'a, T> Iterator for MyIntersection<'a, T>
where
    T: std::cmp::Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self {
            MyIntersection::Stitch { a_iter, b_iter } => {
                let mut a_next = a_iter.next()?;
                let mut b_next = b_iter.next()?;
                loop {
                    match Ord::cmp(a_next, b_next) {
                        std::cmp::Ordering::Less => {
                            a_next = a_iter.next()?;
                        }
                        std::cmp::Ordering::Greater => {
                            b_next = b_iter.next()?;
                        }
                        std::cmp::Ordering::Equal => return Some(a_next),
                    }
                }
            }
            MyIntersection::Spring {
                a_range,
                a_set,
                b_range,
                b_set,
            } => {
                const NEXT_COUNT_MAX: usize = 2;
                let mut next_count: usize = 0;
                let mut a_next = a_range.next()?;
                let mut b_next = b_range.next()?;
                loop {
                    match Ord::cmp(a_next, b_next) {
                        std::cmp::Ordering::Less => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *a_range = a_set.range(b_next..);
                            }
                            a_next = a_range.next()?;
                        }
                        std::cmp::Ordering::Greater => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *b_range = b_set.range(a_next..);
                            }
                            b_next = b_range.next()?;
                        }
                        std::cmp::Ordering::Equal => return Some(a_next),
                    }
                }
            }
            MyIntersection::Search { a_iter, b_set } => loop {
                let a_next = a_iter.next()?;
                if b_set.contains(&a_next) {
                    return Some(a_next);
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max_size = match self {
            MyIntersection::Stitch { a_iter, .. } => a_iter.len(),
            MyIntersection::Spring { a_set, .. } => a_set.len(),
            MyIntersection::Search { a_iter, .. } => a_iter.len(),
        };
        (0, Some(max_size))
    }
}

// Temporary optimization awaiting https://github.com/rust-lang/rust/pull/58577
pub fn btree_intersect<'a, T>(
    selv: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> MyIntersection<'a, T>
where
    T: std::cmp::Ord,
{
    let (a_set, b_set) = if selv.len() <= other.len() {
        (selv, other)
    } else {
        (other, selv)
    };
    if a_set.len() > b_set.len() / 16 {
        MyIntersection::Stitch {
            a_iter: a_set.iter(),
            b_iter: b_set.iter(),
        }
    } else {
        MyIntersection::Search {
            a_iter: a_set.iter(),
            b_set,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    extern crate test;
    use self::rand::{thread_rng, Rng};
    use self::test::{black_box, Bencher};

    #[test]
    fn test_sets_empty_small() {
        let s1: BTreeSet<_> = BTreeSet::new();
        let s2: BTreeSet<_> = (0..15).collect();
        assert_eq!(btree_intersect(&s1, &s2).count(), 0);
    }

    #[test]
    fn test_sets_small_small() {
        let s1: BTreeSet<_> = [5, 10].into_iter().cloned().collect();
        let s2: BTreeSet<_> = (0..15).collect();
        assert_eq!(
            btree_intersect(&s1, &s2).cloned().collect::<Vec<_>>(),
            vec![5, 10]
        );
    }

    #[test]
    fn test_sets_small_medium() {
        let s1: BTreeSet<_> = [5, 10].into_iter().cloned().collect();
        let s2: BTreeSet<_> = (0..42).collect();
        assert_eq!(
            btree_intersect(&s1, &s2).cloned().collect::<Vec<_>>(),
            vec![5, 10]
        );
    }

    #[test]
    fn test_sets_small_large() {
        let s1: BTreeSet<_> = [5, 10].into_iter().cloned().collect();
        let s2: BTreeSet<_> = (0..999).collect();
        assert_eq!(
            btree_intersect(&s1, &s2).cloned().collect::<Vec<_>>(),
            vec![5, 10]
        );
    }

    fn random(n1: usize, n2: usize) -> [BTreeSet<usize>; 2] {
        let mut rng = thread_rng();
        let mut sets = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..2 {
            while sets[i].len() < [n1, n2][i] {
                sets[i].insert(rng.gen());
            }
        }
        assert_eq!(sets[0].len(), n1);
        assert_eq!(sets[1].len(), n2);
        sets
    }

    fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
        let n2 = n1 * factor;
        let mut sets = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..(n1 + n2) {
            let b = i % (factor + 1) != 0;
            sets[b as usize].insert(i as u32);
        }
        assert_eq!(sets[0].len(), n1);
        assert_eq!(sets[1].len(), n2);
        sets
    }

    fn neg_vs_pos(n1: usize, n2: usize) -> [BTreeSet<i32>; 2] {
        let mut neg = BTreeSet::new();
        let mut pos = BTreeSet::new();
        for i in -(n1 as i32)..=-1 {
            neg.insert(i);
        }
        for i in 1..=(n2 as i32) {
            pos.insert(i);
        }
        assert_eq!(neg.len(), n1);
        assert_eq!(pos.len(), n2);
        [neg, pos]
    }

    fn pos_vs_neg(n1: usize, n2: usize) -> [BTreeSet<i32>; 2] {
        let mut sets = neg_vs_pos(n2, n1);
        sets.reverse();
        assert_eq!(sets[0].len(), n1);
        assert_eq!(sets[1].len(), n2);
        sets
    }

    fn intersection_search<T>(sets: &[BTreeSet<T>; 2]) -> MyIntersection<T>
        where T: std::cmp::Ord 
    {
        MyIntersection::Search {
            a_iter: sets[0].iter(),
            b_set: &sets[1],
        }
    }

    fn intersection_spring<T>(sets: &[BTreeSet<T>; 2]) -> MyIntersection<T>
        where T: std::cmp::Ord 
    {
        MyIntersection::Spring {
            a_range: sets[0].range(..),
            a_set: &sets[0],
            b_range: sets[1].range(..),
            b_set: &sets[1],
        }
    }

    fn intersection_stitch<T>(sets: &[BTreeSet<T>; 2]) -> MyIntersection<T>
        where T: std::cmp::Ord 
    {
        MyIntersection::Stitch {
            a_iter: sets[0].iter(),
            b_iter: sets[1].iter(),
        }
    }

    macro_rules! intersection_bench {
        ($name: ident, $sets: expr) => {
            #[bench]
            pub fn $name(b: &mut Bencher) {
                // setup
                let sets = $sets;

                // measure
                b.iter(|| {
                    let x = btree_intersect(&sets[0], &sets[1]).count();
                    black_box(x);
                })
            }
        };
        ($name: ident, $sets: expr, $intersection_kind: ident) => {
            #[bench]
            pub fn $name(b: &mut Bencher) {
                // setup
                let sets = $sets;
                assert!(sets[0].len() >= 1);
                assert!(sets[1].len() >= sets[0].len());

                // measure
                b.iter(|| {
                    let x = $intersection_kind(&sets).count();
                    black_box(x);
                })
            }
        };
    }

    intersection_bench! {intersect_100_neg_vs_100_pos,      neg_vs_pos(100, 100)}
    intersection_bench! {intersect_100_neg_vs_10k_pos,      neg_vs_pos(100, 10_000)}
    intersection_bench! {intersect_100_pos_vs_100_neg,      pos_vs_neg(100, 100)}
    intersection_bench! {intersect_100_pos_vs_10k_neg,      pos_vs_neg(100, 10_000)}
    intersection_bench! {intersect_10k_neg_vs_100_pos,      neg_vs_pos(10_000, 100)}
    intersection_bench! {intersect_10k_neg_vs_10k_pos,      neg_vs_pos(10_000, 10_000)}
    intersection_bench! {intersect_10k_pos_vs_100_neg,      pos_vs_neg(10_000, 100)}
    intersection_bench! {intersect_10k_pos_vs_10k_neg,      pos_vs_neg(10_000, 10_000)}
    intersection_bench! {intersect_random_100_vs_100_actual,random(100, 100)}
    intersection_bench! {intersect_random_100_vs_100_search,random(100, 100), intersection_search}
    intersection_bench! {intersect_random_100_vs_100_spring,random(100, 100), intersection_spring}
    intersection_bench! {intersect_random_100_vs_100_stitch,random(100, 100), intersection_stitch}
    intersection_bench! {intersect_random_100_vs_10k_actual,random(100, 10_000)}
    intersection_bench! {intersect_random_100_vs_10k_search,random(100, 10_000), intersection_search}
    intersection_bench! {intersect_random_100_vs_10k_spring,random(100, 10_000), intersection_spring}
    intersection_bench! {intersect_random_100_vs_10k_stitch,random(100, 10_000), intersection_stitch}
    intersection_bench! {intersect_random_10k_vs_10k_actual,random(10_000, 10_000)}
    intersection_bench! {intersect_random_10k_vs_10k_search,random(10_000, 10_000), intersection_search}
    intersection_bench! {intersect_random_10k_vs_10k_spring,random(10_000, 10_000), intersection_spring}
    intersection_bench! {intersect_random_10k_vs_10k_stitch,random(10_000, 10_000), intersection_stitch}
    intersection_bench! {intersect_stagger_100_actual,      stagger(100, 1)}
    intersection_bench! {intersect_stagger_100_search,      stagger(100, 1), intersection_search}
    intersection_bench! {intersect_stagger_100_spring,      stagger(100, 1), intersection_spring}
    intersection_bench! {intersect_stagger_100_stitch,      stagger(100, 1), intersection_stitch}
    intersection_bench! {intersect_stagger_10k_actual,      stagger(10_000, 1)}
    intersection_bench! {intersect_stagger_10k_search,      stagger(10_000, 1), intersection_search}
    intersection_bench! {intersect_stagger_10k_spring,      stagger(10_000, 1), intersection_spring}
    intersection_bench! {intersect_stagger_10k_stitch,      stagger(10_000, 1), intersection_stitch}
    intersection_bench! {intersect_stagger_1_actual,        stagger(1, 1)}
    intersection_bench! {intersect_stagger_1_search,        stagger(1, 1), intersection_search}
    intersection_bench! {intersect_stagger_1_spring,        stagger(1, 1), intersection_spring}
    intersection_bench! {intersect_stagger_1_stitch,        stagger(1, 1), intersection_stitch}
    intersection_bench! {intersect_stagger_diff1_actual,    stagger(100, 1 << 1)}
    intersection_bench! {intersect_stagger_diff1_search,    stagger(100, 1 << 1), intersection_search}
    intersection_bench! {intersect_stagger_diff1_spring,    stagger(100, 1 << 1), intersection_spring}
    intersection_bench! {intersect_stagger_diff1_stitch,    stagger(100, 1 << 1), intersection_stitch}
    intersection_bench! {intersect_stagger_diff2_actual,    stagger(100, 1 << 2)}
    intersection_bench! {intersect_stagger_diff2_search,    stagger(100, 1 << 2), intersection_search}
    intersection_bench! {intersect_stagger_diff2_spring,    stagger(100, 1 << 2), intersection_spring}
    intersection_bench! {intersect_stagger_diff2_stitch,    stagger(100, 1 << 2), intersection_stitch}
    intersection_bench! {intersect_stagger_diff3_actual,    stagger(100, 1 << 3)}
    intersection_bench! {intersect_stagger_diff3_search,    stagger(100, 1 << 3), intersection_search}
    intersection_bench! {intersect_stagger_diff3_spring,    stagger(100, 1 << 3), intersection_spring}
    intersection_bench! {intersect_stagger_diff3_stitch,    stagger(100, 1 << 3), intersection_stitch}
    intersection_bench! {intersect_stagger_diff4_actual,    stagger(100, 1 << 4)}
    intersection_bench! {intersect_stagger_diff4_search,    stagger(100, 1 << 4), intersection_search}
    intersection_bench! {intersect_stagger_diff4_spring,    stagger(100, 1 << 4), intersection_spring}
    intersection_bench! {intersect_stagger_diff4_stitch,    stagger(100, 1 << 4), intersection_stitch}
    intersection_bench! {intersect_stagger_diff5_actual,    stagger(100, 1 << 5)}
    intersection_bench! {intersect_stagger_diff5_search,    stagger(100, 1 << 5), intersection_search}
    intersection_bench! {intersect_stagger_diff5_spring,    stagger(100, 1 << 5), intersection_spring}
    intersection_bench! {intersect_stagger_diff5_stitch,    stagger(100, 1 << 5), intersection_stitch}
    intersection_bench! {intersect_stagger_diff6_actual,    stagger(100, 1 << 6)}
    intersection_bench! {intersect_stagger_diff6_search,    stagger(100, 1 << 6), intersection_search}
    intersection_bench! {intersect_stagger_diff6_spring,    stagger(100, 1 << 6), intersection_spring}
    intersection_bench! {intersect_stagger_diff6_stitch,    stagger(100, 1 << 6), intersection_stitch}
    }
