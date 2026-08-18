use crate::Clique;

pub trait CliqueAccumulator {
    type Harvest;
    fn add(&mut self, clique: Clique);
    fn absorb(&mut self, other: Self);
    fn harvest(self) -> Self::Harvest;
}
