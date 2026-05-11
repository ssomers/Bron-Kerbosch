//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::algorithm::BronKerboschAlgorithm;
use super::bron_kerbosch_degen::{PivotChoice, explore_with_degeneracy};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver3½-GPX")
    }

    fn explore<VertexSet, Consumer>(
        graph: &Graph<VertexSet>,
        consumer: Consumer,
    ) -> Consumer::Harvest
    where
        VertexSet: VertexSetLike,
        Consumer: CliqueConsumer,
    {
        explore_with_degeneracy(graph, consumer, PivotChoice::MaxDegreeLocalX)
    }
}
