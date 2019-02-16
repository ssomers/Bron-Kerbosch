use graph::Vertex;

use std::collections::BTreeSet;

pub type Clique = Vec<Vertex>;

pub trait Reporter {
    fn record(&mut self, clique: Clique);
}

#[derive(Debug)]
pub struct SimpleReporter {
    pub cliques: Vec<Clique>,
}

impl SimpleReporter {
    pub fn new() -> SimpleReporter {
        SimpleReporter { cliques: vec![] }
    }
}

impl Reporter for SimpleReporter {
    fn record(&mut self, clique: Clique) {
        debug_assert!(clique.len() > 1);
        debug_assert_eq!(
            clique.iter().cloned().collect::<BTreeSet<Vertex>>().len(),
            clique.len()
        );
        self.cliques.push(clique);
    }
}
