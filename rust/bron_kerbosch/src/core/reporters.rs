pub use crate::core::reporter::{Clique, Reporter};
use crate::core::vertex::Vertex;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct CollectingReporter {
    pub cliques: Vec<Clique>,
}

impl Reporter for CollectingReporter {
    fn record(&mut self, clique: Clique) {
        debug_assert!(clique.len() > 1);
        debug_assert_eq!(
            clique.iter().copied().collect::<BTreeSet<Vertex>>().len(),
            clique.len()
        );
        self.cliques.push(clique);
    }
}

#[derive(Debug, Default)]
pub struct CountingReporter {
    pub count: usize,
}

impl Reporter for CountingReporter {
    fn record(&mut self, _clique: Clique) {
        self.count += 1;
    }
}
