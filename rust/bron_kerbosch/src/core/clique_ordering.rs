use super::clique::{Clique, Vertex};
use std::collections::BTreeSet;

pub type OrderedClique = BTreeSet<Vertex>;
pub type OrderedCliques = BTreeSet<OrderedClique>;
pub fn order_cliques<I: Iterator<Item = Clique>>(cliques: I) -> OrderedCliques {
    BTreeSet::from_iter(cliques.map(BTreeSet::from_iter))
}
