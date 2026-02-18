use super::vertex::Vertex;

pub type Clique = Vec<Vertex>;

pub trait CliqueConsumer {
    fn accept(&mut self, clique: Clique);
}
