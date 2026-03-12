use bron_kerbosch::{Clique, CliqueConsumer};

#[derive(Debug, Default)]
pub struct CliqueCounter {
    pub count: usize,
}

impl CliqueConsumer for CliqueCounter {
    fn accept(&mut self, _clique: Clique) {
        self.count += 1;
    }
}
