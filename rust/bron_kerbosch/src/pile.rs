pub struct Pile<'a, T> {
    last: PileLayer<'a, T>,
}

enum PileLayer<'a, T> {
    Empty,
    Cons(&'a PileLayer<'a, T>, T),
}

impl<'a, T> PileLayer<'a, T>
where
    T: Clone,
{
    fn len(&self) -> usize {
        match self {
            PileLayer::Empty => 0,
            PileLayer::Cons(preceding, _t) => preceding.len() + 1,
        }
    }

    fn push_to(&self, result: &mut Vec<T>) {
        match self {
            PileLayer::Empty => {}
            PileLayer::Cons(preceding, t) => {
                preceding.push_to(result);
                result.push(t.clone());
            }
        }
    }
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Pile {
            last: PileLayer::Empty,
        }
    }

    pub fn from(t: T) -> Self {
        Pile {
            last: PileLayer::Cons(&PileLayer::Empty, t),
        }
    }

    pub fn cons(&'a self, t: T) -> Self {
        Self {
            last: PileLayer::Cons::<'a>(&self.last, t),
        }
    }

    pub fn collect(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.last.len());
        self.last.push_to(&mut result);
        result
    }
}
