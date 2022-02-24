use crate::graph::Vertex;
use crate::reporter::{Clique, Reporter};

use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct SimpleReporter {
    pub cliques: Vec<Clique>,
}

impl Reporter for SimpleReporter {
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
