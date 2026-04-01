use crate::core::clique::Clique;
use crate::core::clique_consumer::CliqueConsumer;
use crossbeam_channel::Receiver;

#[derive(Debug)]
pub struct CliqueHarvester {
    consumer_rx: Receiver<Clique>,
}

pub fn new_clique_channel(
    channel_cap: usize,
    min_size: usize,
) -> (CliqueConsumer, CliqueHarvester) {
    let (consumer_tx, consumer_rx) = crossbeam_channel::bounded::<Clique>(channel_cap);
    let consumer = CliqueConsumer::new(min_size, consumer_tx);
    (consumer, CliqueHarvester { consumer_rx })
}

impl CliqueHarvester {
    pub fn collect_cliques(self) -> Vec<Clique> {
        let mut cliques = vec![];
        while let Ok(clique) = self.consumer_rx.recv() {
            cliques.push(clique);
        }
        cliques
    }

    pub fn count_cliques(self) -> usize {
        let mut cliques = 0;
        while self.consumer_rx.recv().is_ok() {
            cliques += 1;
        }
        cliques
    }
}
