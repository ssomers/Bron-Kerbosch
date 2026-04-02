use super::vertex::Vertex;
use crossbeam_channel::Sender;

pub type Clique = Vec<Vertex>;

#[derive(Clone, Debug)]
pub struct CliqueConsumer {
    min_size: usize,
    consumer_tx: Sender<Clique>,
}

impl CliqueConsumer {
    pub fn new(min_size: usize, consumer_tx: Sender<Clique>) -> Self {
        assert!(min_size >= 2); // We don't want to write code for the 0-clique or 1-cliques.
        Self {
            min_size,
            consumer_tx,
        }
    }

    pub fn is_accepted_size(&self, size: usize) -> bool {
        size >= self.min_size
    }
    pub fn accept(&mut self, clique: Clique) {
        debug_assert!(self.is_accepted_size(clique.len()));
        self.consumer_tx.send(clique).unwrap();
    }
}
