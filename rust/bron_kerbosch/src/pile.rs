/// Cheap & simple immutable stack data structure, of which the layers
/// somehow live long enough (e.g. they are on the runtime stack).
/// The only supported change is to add, and the only supported query is to
/// convert to a vector.
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

    /// Create a pile on top of an existing pile
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

impl<'a, T> Default for Pile<'a, T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pile() {
        let p0: Pile<i32> = Pile::new();
        {
            let p1 = p0.place(4);
            {
                let p2 = p1.place(2);
                assert_eq!(p2.collect(), vec![4, 2]);
            }
            assert_eq!(p1.collect(), vec![4]);
        }
        assert_eq!(p0.collect(), vec![]);
    }
}
