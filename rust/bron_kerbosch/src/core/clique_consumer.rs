use crate::{Clique, CliqueAccumulator};

pub struct CliqueConsumer<'a, Accumulator: CliqueAccumulator> {
    pub min_clique_size: usize,
    pub accu: &'a mut Accumulator,
}

impl<'a, Accumulator: CliqueAccumulator> CliqueConsumer<'a, Accumulator> {
    pub fn accept(&mut self, clique: Clique) {
        debug_assert!(clique.len() >= self.min_clique_size);
        self.accu.add(clique);
    }
}
