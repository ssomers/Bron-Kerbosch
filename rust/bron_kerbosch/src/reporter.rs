use crate::graph::Vertex;

pub type Clique = Vec<Vertex>;

pub trait Reporter {
    fn record(&mut self, clique: Clique);
}
