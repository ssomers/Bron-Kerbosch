use crate::core::clique::Clique;
use crate::core::clique_consumer::CliqueConsumer;

#[derive(Debug)]
pub struct CliqueCollector {
    min_size: usize,
    cliques: Vec<Clique>,
}

#[derive(Debug)]
pub struct CliqueCounter {
    min_size: usize,
    cliques: usize,
}

impl CliqueCollector {
    pub fn new(min_size: usize) -> Self {
        Self {
            min_size,
            cliques: vec![],
        }
    }
    pub fn harvest(self) -> Vec<Clique> {
        self.cliques
    }
}
impl CliqueConsumer for CliqueCollector {
    type Harvest = Vec<Clique>;

    fn is_accepted_size(&self, size: usize) -> bool {
        size >= self.min_size
    }
    fn accept(&mut self, clique: Clique) {
        self.cliques.push(clique)
    }
    fn harvest(self) -> Vec<Clique> {
        self.cliques
    }
    fn combine(cliques1: Vec<Clique>, cliques2: Vec<Clique>) -> Vec<Clique> {
        cliques1.into_iter().chain(cliques2).collect()
    }
}
impl Clone for CliqueCollector {
    fn clone(&self) -> Self {
        assert!(self.cliques.is_empty());
        Self::new(self.min_size)
    }
}

impl CliqueCounter {
    pub fn new(min_size: usize) -> Self {
        Self {
            min_size,
            cliques: 0,
        }
    }
    pub fn harvest(self) -> usize {
        self.cliques
    }
}
impl CliqueConsumer for CliqueCounter {
    type Harvest = usize;

    fn is_accepted_size(&self, size: usize) -> bool {
        size >= self.min_size
    }
    fn accept(&mut self, _: Clique) {
        self.cliques += 1
    }
    fn harvest(self) -> usize {
        self.cliques
    }
    fn combine(cliques1: usize, cliques2: usize) -> usize {
        cliques1 + cliques2
    }
}
impl Clone for CliqueCounter {
    fn clone(&self) -> Self {
        assert_eq!(self.cliques, 0);
        Self::new(self.min_size)
    }
}
