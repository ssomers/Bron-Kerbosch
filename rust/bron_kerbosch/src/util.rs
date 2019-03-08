extern crate core;
use std::collections::BTreeSet;
use util::core::cmp::min;
use util::core::iter::Peekable;

enum MyIntersectionOther<'a, T> {
    ITER(Peekable<std::collections::btree_set::Iter<'a, T>>),
    SET(&'a BTreeSet<T>),
}
pub struct MyIntersection<'a, T> {
    a: Peekable<std::collections::btree_set::Iter<'a, T>>,
    b: MyIntersectionOther<'a, T>,
}

impl<'a, T> Iterator for MyIntersection<'a, T>
where
    T: std::cmp::Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.b {
            MyIntersectionOther::ITER(ref mut self_b) => loop {
                match Ord::cmp(self.a.peek()?, self_b.peek()?) {
                    std::cmp::Ordering::Less => {
                        self.a.next();
                    }
                    std::cmp::Ordering::Equal => {
                        self_b.next();
                        return self.a.next();
                    }
                    std::cmp::Ordering::Greater => {
                        self_b.next();
                    }
                }
            },
            MyIntersectionOther::SET(set) => loop {
                match self.a.next() {
                    None => return None,
                    Some(e) => {
                        if set.contains(&e) {
                            return Some(e);
                        }
                    }
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let b_len = match self.b {
            MyIntersectionOther::ITER(ref iter) => iter.len(),
            MyIntersectionOther::SET(set) => set.len(),
        };
        (0, Some(min(self.a.len(), b_len)))
    }
}

fn are_proportionate_for_intersection(len1: usize, len2: usize) -> bool {
    let (small, large) = if len1 <= len2 {
        (len1, len2)
    } else {
        (len2, len1)
    };
    (large >> 4) <= small
}

// Temporary optimization awaiting https://github.com/rust-lang/rust/pull/58577
pub fn btree_intersect<'a, T>(
    selv: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> MyIntersection<'a, T>
where
    T: std::cmp::Ord,
{
    if are_proportionate_for_intersection(selv.len(), other.len()) {
        MyIntersection {
            a: selv.iter().peekable(),
            b: MyIntersectionOther::ITER(other.iter().peekable()),
        }
    } else if selv.len() <= other.len() {
        MyIntersection {
            a: selv.iter().peekable(),
            b: MyIntersectionOther::SET(&other),
        }
    } else {
        MyIntersection {
            a: other.iter().peekable(),
            b: MyIntersectionOther::SET(&selv),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_proportionate_for_intersection() {
        assert!(are_proportionate_for_intersection(0, 0));
        assert!(are_proportionate_for_intersection(0, 15));
        assert!(!are_proportionate_for_intersection(0, 16));
        assert!(are_proportionate_for_intersection(1, 31));
        assert!(!are_proportionate_for_intersection(1, 32));
        assert!(are_proportionate_for_intersection(15, 0));
        assert!(!are_proportionate_for_intersection(16, 0));
        assert!(are_proportionate_for_intersection(31, 1));
        assert!(!are_proportionate_for_intersection(32, 1));
    }
}
