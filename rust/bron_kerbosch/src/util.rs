use std::collections::HashSet;

pub fn intersect<'a, T, S>(
    s1: &'a HashSet<T, S>,
    s2: &'a HashSet<T, S>,
) -> std::collections::hash_set::Intersection<'a, T, S>
where
    T: Eq + std::hash::Hash,
    S: std::hash::BuildHasher,
{
    if s1.len() < s2.len() {
        s1.intersection(s2)
    } else {
        s2.intersection(s1)
    }
}
