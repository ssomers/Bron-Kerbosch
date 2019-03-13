pub struct Pile<'a, T> {
    layers: PileLayer<'a, T>,
}

enum PileLayer<'a, T> {
    Empty,
    Cons { top: T, below: &'a PileLayer<'a, T> },
}

impl<'a, T> PileLayer<'a, T>
where
    T: Clone,
{
    fn height(&self) -> usize {
        match self {
            PileLayer::Empty => 0,
            PileLayer::Cons { below, .. } => below.height() + 1,
        }
    }

    fn push_to(&self, result: &mut Vec<T>) {
        if let PileLayer::Cons { top, below } = self {
            below.push_to(result);
            result.push(top.clone());
        }
    }
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            layers: PileLayer::Empty,
        }
    }

    pub fn from(t: T) -> Self {
        Self {
            layers: PileLayer::Cons {
                top: t,
                below: &PileLayer::Empty,
            },
        }
    }

    pub fn place(&'a self, top: T) -> Self {
        Self {
            layers: PileLayer::Cons {
                top,
                below: &self.layers,
            },
        }
    }

    pub fn collect(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.layers.height());
        self.layers.push_to(&mut result);
        result
    }
}
