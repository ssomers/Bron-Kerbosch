use crate::CliqueAccumulator;
use crate::core::clique::Clique;

#[derive(Debug, Default)]
pub struct CliqueCollector {
    cliques: Vec<Clique>,
}

#[derive(Debug, Default)]
pub struct CliqueCounter {
    cliques: usize,
}

impl CliqueAccumulator for CliqueCollector {
    type Harvest = Vec<Clique>;

    fn add(&mut self, clique: Clique) {
        self.cliques.push(clique)
    }
    fn absorb(&mut self, mut other: Self) {
        self.cliques.append(&mut other.cliques)
    }
    fn harvest(self) -> Vec<Clique> {
        self.cliques
    }
}
impl Clone for CliqueCollector {
    fn clone(&self) -> Self {
        assert!(self.cliques.is_empty());
        Self::default()
    }
}

impl CliqueAccumulator for CliqueCounter {
    type Harvest = usize;

    fn add(&mut self, _: Clique) {
        self.cliques += 1
    }
    fn absorb(&mut self, other: Self) {
        self.cliques += other.cliques
    }
    fn harvest(self) -> usize {
        self.cliques
    }
}
impl Clone for CliqueCounter {
    fn clone(&self) -> Self {
        assert_eq!(self.cliques, 0);
        Self::default()
    }
}
