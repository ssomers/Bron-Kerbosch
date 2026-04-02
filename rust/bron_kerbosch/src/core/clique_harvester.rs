use crate::core::clique::{Clique, CliqueConsumer};
use crossbeam_channel::Receiver;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct CliqueHarvester {
    min_size: usize,
    consumer_rx: Receiver<Clique>,
}

impl CliqueHarvester {
    pub fn new(channel_cap: usize, clique_min_size: usize) -> (CliqueConsumer, Self) {
        let (consumer_tx, consumer_rx) = crossbeam_channel::bounded::<Clique>(channel_cap);
        (
            CliqueConsumer::new(clique_min_size, consumer_tx),
            Self {
                min_size: clique_min_size,
                consumer_rx,
            },
        )
    }

    pub fn collect_cliques(self) -> Vec<Clique> {
        let mut cliques = vec![];
        while let Ok(clique) = self.consumer_rx.recv() {
            debug_assert!(self.is_valid(&clique));
            cliques.push(clique);
        }
        cliques
    }

    pub fn count_cliques(self) -> usize {
        let mut cliques = 0;
        while let Ok(clique) = self.consumer_rx.recv() {
            debug_assert!(self.is_valid(&clique));
            cliques += 1;
        }
        cliques
    }

    fn is_valid(&self, clique: &Clique) -> bool {
        clique.len() == count_unique_elements(clique) && clique.len() >= self.min_size
    }
}

pub fn count_unique_elements<T: Copy + Eq + Hash>(v: &[T]) -> usize {
    v.iter().copied().collect::<HashSet<_>>().len()
}
