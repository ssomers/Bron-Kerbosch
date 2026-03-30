pub use super::vertex::Vertex;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct Clique(Vec<Vertex>);

impl Clique {
    pub const EMPTY: Clique = Clique(vec![]);

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(&self, v: Vertex) -> Clique {
        debug_assert!(!self.0.contains(&v));
        Clique([self.0.as_slice(), &[v]].concat())
    }
}

impl FromIterator<Vertex> for Clique {
    fn from_iter<I: IntoIterator<Item = Vertex>>(iter: I) -> Self {
        let vertices = Vec::from_iter(iter);
        debug_assert!(vertices.len() == count_unique_elements(&vertices));
        Clique(vertices)
    }
}

impl IntoIterator for Clique {
    type Item = Vertex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn count_unique_elements<T: Copy + Eq + Hash>(v: &[T]) -> usize {
    v.iter().copied().collect::<HashSet<_>>().len()
}
