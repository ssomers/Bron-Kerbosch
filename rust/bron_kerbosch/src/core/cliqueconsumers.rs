pub use crate::core::base::{Clique, CliqueConsumer};
use crate::core::vertex::Vertex;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct CliqueCollector {
    pub cliques: Vec<Clique>,
}

impl CliqueConsumer for CliqueCollector {
    fn accept(&mut self, clique: Clique) {
        debug_assert!(clique.len() > 1);
        debug_assert_eq!(
            clique.iter().copied().collect::<BTreeSet<Vertex>>().len(),
            clique.len()
        );
        self.cliques.push(clique);
    }
}

#[derive(Debug, Default)]
pub struct CliqueCounter {
    pub count: usize,
}

impl CliqueConsumer for CliqueCounter {
    fn accept(&mut self, _clique: Clique) {
        self.count += 1;
    }
}
