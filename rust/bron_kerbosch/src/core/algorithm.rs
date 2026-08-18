use crate::{CliqueAccumulator, Graph, VertexSetLike};

pub trait BronKerboschAlgorithm {
    fn name() -> String;

    fn deterministic() -> bool {
        true
    }

    fn explore<VertexSet, Accumulator>(
        graph: &Graph<VertexSet>,
        min_clique_size: usize,
        accumulator: Accumulator,
    ) -> Accumulator::Harvest
    where
        VertexSet: VertexSetLike + Sync,
        Accumulator: CliqueAccumulator + Clone + Send;
}
