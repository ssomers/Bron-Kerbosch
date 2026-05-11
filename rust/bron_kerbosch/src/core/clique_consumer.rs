use super::clique::Clique;

pub trait CliqueConsumer {
    type Harvest;
    fn min_size(&self) -> usize;
    fn accept(&mut self, clique: Clique);
    fn harvest(self) -> Self::Harvest;
    fn combine(a: Self::Harvest, b: Self::Harvest) -> Self::Harvest;
}
