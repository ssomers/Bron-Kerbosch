use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub trait BronKerboschAlgorithm {
    fn name() -> String;

    fn deterministic() -> bool {
        true
    }

    fn explore<VertexSet, Consumer>(
        graph: &Graph<VertexSet>,
        consumer: Consumer,
    ) -> Consumer::Harvest
    where
        VertexSet: VertexSetLike + Sync,
        Consumer: CliqueConsumer + Clone + Send;
}
