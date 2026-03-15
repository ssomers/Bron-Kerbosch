use super::vertex::Vertex;
use crossbeam_channel::Sender;

pub type Clique = Vec<Vertex>;

#[derive(Clone, Debug)]
pub struct CliqueConsumer {
    consumer_tx: Sender<Clique>,
}

impl CliqueConsumer {
    pub fn new(consumer_tx: Sender<Clique>) -> Self {
        Self { consumer_tx }
    }

    pub fn accept(&mut self, clique: Clique) {
        debug_assert!(clique.len() >= 2);
        self.consumer_tx.send(clique).unwrap();
    }
}
