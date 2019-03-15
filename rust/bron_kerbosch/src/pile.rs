pub struct Pile<'a, T> {
    layers: PileLayer<'a, T>,
}

enum PileLayer<'a, T> {
    Empty,
    Cons {
        earlier: &'a PileLayer<'a, T>,
        last: T,
    },
}

impl<'a, T> PileLayer<'a, T>
where
    T: Clone,
{
    fn height(&self) -> usize {
        match self {
            PileLayer::Empty => 0,
            PileLayer::Cons { earlier, .. } => earlier.height() + 1,
        }
    }

    fn push_to(&self, result: &mut Vec<T>) {
        if let PileLayer::Cons { earlier, last } = self {
            earlier.push_to(result);
            result.push(last.clone());
        }
    }
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    /// Create empty pile
    pub fn new() -> Self {
        Self {
            layers: PileLayer::Empty,
        }
    }

    /// Create a pile containing one element
    pub fn from(t: T) -> Self {
        Self {
            layers: PileLayer::Cons {
                earlier: &PileLayer::Empty,
                last: t,
            },
        }
    }

    /// Place additional element on a pile
    pub fn place(&'a self, last: T) -> Self {
        Self {
            layers: PileLayer::Cons {
                earlier: &self.layers,
                last,
            },
        }
    }

    /// Clone contained elements into a vector in the order they were placed
    pub fn collect(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.layers.height());
        self.layers.push_to(&mut result);
        result
    }
}
