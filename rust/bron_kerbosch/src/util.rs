use std::collections::HashSet;

pub fn intersect<'a, T, S>(
    s1: &'a HashSet<T, S>,
    s2: &'a HashSet<T, S>,
) -> std::collections::hash_set::Intersection<'a, T, S>
where
    T: Eq + std::hash::Hash,
    S: std::hash::BuildHasher,
{
    if s1.len() <= s2.len() {
        s1.intersection(s2)
    } else {
        s2.intersection(s1)
    }
}

pub fn pop_arbitrary<T>(s: &mut HashSet<T>) -> Option<T>
where
    T: Eq + Clone + std::hash::Hash,
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
        let mut s: HashSet<u32> = [4, 2].iter().cloned().collect();
        assert!(pop_arbitrary(&mut s).is_some());
        assert_eq!(s.len(), 1);
        assert!(pop_arbitrary(&mut s).is_some());
        assert_eq!(s.len(), 0);
        assert!(pop_arbitrary(&mut s).is_none());
    }
}
