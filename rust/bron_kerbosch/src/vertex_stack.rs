type Vertex = u32;

pub enum VertexStack<'a> {
    Empty,
    Cons(&'a VertexStack<'a>, Vertex),
}

impl<'a> VertexStack<'a> {
    pub fn collect(&self) -> Vec<Vertex> {
        let mut clique: Vec<Vertex> = Vec::with_capacity(self.len());
        self.append_to(&mut clique);
        clique
    }

    fn len(&self) -> usize {
        match self {
            VertexStack::Empty => 0,
            VertexStack::Cons(c, _v) => c.len() + 1,
        }
    }

    fn append_to(&self, clique: &mut Vec<Vertex>) {
        match self {
            VertexStack::Empty => {}
            VertexStack::Cons(c, v) => {
                c.append_to(clique);
                clique.push(*v);
            }
        }
    }
}
