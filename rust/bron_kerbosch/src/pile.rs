pub enum Pile<'a, T> {
    Empty,
    Cons(&'a Pile<'a, T>, T),
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    pub fn collect(&self) -> Vec<T> {
        let mut clique: Vec<T> = Vec::with_capacity(self.len());
        self.append_to(&mut clique);
        clique
    }

    fn len(&self) -> usize {
        match self {
            Pile::Empty => 0,
            Pile::Cons(c, _v) => c.len() + 1,
        }
    }

    fn append_to(&self, clique: &mut Vec<T>) {
        match self {
            Pile::Empty => {}
            Pile::Cons(c, t) => {
                c.append_to(clique);
                clique.push(t.clone());
            }
        }
    }
}
