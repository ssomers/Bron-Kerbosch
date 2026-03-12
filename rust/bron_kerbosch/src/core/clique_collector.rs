use crate::core::clique::{Clique, CliqueConsumer};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct CliqueCollector {
    pub cliques: Vec<Clique>,
}

pub fn count_unique_elements<T: Copy + Eq + Hash>(v: &[T]) -> usize {
    v.iter().copied().collect::<HashSet<_>>().len()
}

impl CliqueConsumer for CliqueCollector {
    fn accept(&mut self, clique: Clique) {
        assert!(clique.len() > 1);
        debug_assert_eq!(clique.len(), count_unique_elements(&clique));
        self.cliques.push(clique);
    }
}
