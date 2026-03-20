use super::clique::{Clique, Vertex};
use std::collections::BTreeSet;

pub type OrderedCliques = BTreeSet<BTreeSet<Vertex>>;
pub fn order_cliques<I: Iterator<Item = Clique>>(cliques: I) -> OrderedCliques {
    BTreeSet::from_iter(cliques.map(BTreeSet::from_iter))
}
