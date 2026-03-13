/// Cheap & simple immutable stack data structure, of which the layers
/// somehow live long enough (e.g. they are on the runtime stack).
/// The only supported change is to add, and the only supported query is to
/// convert to a vector.
pub struct Pile<'a, T> {
    pub height: usize,
    layers: Option<Layer<'a, T>>,
}
struct Layer<'a, T> {
    top: T,
    lower: &'a Pile<'a, T>,
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    pub const EMPTY: Pile<'a, T> = Pile {
        height: 0,
        layers: None,
    };

    fn push_to(&self, result: &mut Vec<T>) {
        if let Some(layers) = &self.layers {
            layers.lower.push_to(result);
            result.push(layers.top.clone());
        }
    }

    /// Create a pile containing one element.
    pub fn from(t: T) -> Self {
        Pile::EMPTY.pile(t)
    }

    /// Create a pile on top of an existing pile.
    pub fn pile(&'a self, top: T) -> Self {
        Self {
            height: 1 + self.height,
            layers: Some(Layer { top, lower: self }),
        }
    }

    /// Clone contained elements into a vector, in the order they were piled up.
    /// Take ownership because we collect right after piling up the last vertex.
    pub fn collect(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.height);
        self.push_to(&mut result);
        debug_assert_eq!(result.len(), self.height);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pile() {
        let p1 = Pile::from(4);
        {
            let p2 = p1.pile(2);
            assert_eq!(p2.collect(), vec![4, 2]);
        }
        assert_eq!(p1.collect(), vec![4]);
    }
}
