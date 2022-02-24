use super::vertex::Vertex;

pub type Clique = Vec<Vertex>;

pub trait Reporter {
    fn record(&mut self, clique: Clique);
}
