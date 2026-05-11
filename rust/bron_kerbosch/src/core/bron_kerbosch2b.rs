//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::algorithm::BronKerboschAlgorithm;
use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver2½")
    }

    fn explore<VertexSet, Consumer>(
        graph: &Graph<VertexSet>,
        consumer: Consumer,
    ) -> Consumer::Harvest
    where
        VertexSet: VertexSetLike,
        Consumer: CliqueConsumer,
    {
        explore_with_pivot(graph, consumer, PivotChoice::Arbitrary)
    }
}
