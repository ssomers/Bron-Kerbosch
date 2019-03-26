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
                        std::cmp::Ordering::Less => a_next = a_iter.next()?,
                        std::cmp::Ordering::Greater => b_next = b_iter.next()?,
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

// Temporary optimization awaiting https://github.com/rust-lang/rust/pull/59186
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
        /*
        MyIntersection::Spring {
            a_range: a_set.range(..),
            b_range: b_set.range(..),
            a_set,
            b_set,
        }
        */
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
}

#[cfg(test)]
mod proptests {
    extern crate proptest;
    extern crate rand;
    extern crate rand_chacha;
    use self::proptest::prelude::*;
    use self::rand::Rng;
    use self::rand::SeedableRng;
    use self::rand_chacha::ChaChaRng;
    use super::*;

    fn random_set(size: usize, ovule: u8) -> BTreeSet<usize> {
        let mut rng = ChaChaRng::from_seed([ovule; 32]);
        let mut s = BTreeSet::<usize>::new();
        while s.len() < size {
            s.insert(rng.gen());
        }
        s
    }

    proptest! {
        #[test]
        fn intersection(len1 in 0..1000usize, len2 in 0..1000usize) {
            let s1 = random_set(len1, 11u8);
            let s2 = random_set(len2, 22u8);
            let mut collected = BTreeSet::<usize>::new();
            for elt in btree_intersect(&s1, &s2) {
                assert!(collected.insert(*elt));
            }
            for elt in &s1 {
                assert_eq!(collected.contains(elt), s2.contains(elt));
            }
            for elt in &s2 {
                assert_eq!(collected.contains(elt), s1.contains(elt));
            }
        }
    }
}
