extern crate core;
use std::collections::BTreeSet;
use util::core::cmp::min;
use util::core::iter::Peekable;

pub fn is_disjoint<'a, T>(selv: &'a BTreeSet<T>, other: &'a BTreeSet<T>) -> bool
where
    T: std::cmp::Ord,
{
    selv.is_disjoint(other)
}

pub struct MyIntersection<'a, T> {
    a: Peekable<std::collections::btree_set::Iter<'a, T>>,
    b: Peekable<std::collections::btree_set::Iter<'a, T>>,
    big: Option<&'a BTreeSet<T>>,
}

impl<'a, T> Iterator for MyIntersection<'a, T>
where
    T: std::cmp::Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.big {
            None => loop {
                match Ord::cmp(self.a.peek()?, self.b.peek()?) {
                    std::cmp::Ordering::Less => {
                        self.a.next();
                    }
                    std::cmp::Ordering::Equal => {
                        self.b.next();
                        return self.a.next();
                    }
                    std::cmp::Ordering::Greater => {
                        self.b.next();
                    }
                }
            },
            Some(big) => loop {
                match self.a.next() {
                    None => return None,
                    Some(e) => {
                        if big.contains(&e) {
                            return Some(e);
                        }
                    }
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(min(self.a.len(), self.b.len())))
    }
}

fn have_comparable_len<T>(selv: &BTreeSet<T>, other: &BTreeSet<T>) -> bool {
    let (min, delta) = if selv.len() <= other.len() {
        (selv.len(), other.len() - selv.len())
    } else {
        (other.len(), selv.len() - other.len())
    };
    delta / 128 <= min
}

pub fn intersect<'a, T>(selv: &'a BTreeSet<T>, other: &'a BTreeSet<T>) -> MyIntersection<'a, T> {
    if have_comparable_len(selv, other) {
        MyIntersection {
            a: selv.iter().peekable(),
            b: other.iter().peekable(),
            big: None,
        }
    } else if selv.len() <= other.len() {
        MyIntersection {
            a: selv.iter().peekable(),
            b: selv.iter().peekable(), // not used
            big: Some(&other),
        }
    } else {
        MyIntersection {
            a: other.iter().peekable(),
            b: other.iter().peekable(), // not used
            big: Some(&selv),
        }
    }
}

pub fn pop_arbitrary<T>(s: &mut BTreeSet<T>) -> Option<T>
where
    T: std::cmp::Ord + Clone,
{
    s.iter().next().cloned().map(|v| {
        s.remove(&v);
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_arbitrary() {
        let mut s: BTreeSet<u32> = [4, 2].iter().cloned().collect();
        assert!(pop_arbitrary(&mut s).is_some());
        assert_eq!(s.len(), 1);
        assert!(pop_arbitrary(&mut s).is_some());
        assert_eq!(s.len(), 0);
        assert!(pop_arbitrary(&mut s).is_none());
    }
}
