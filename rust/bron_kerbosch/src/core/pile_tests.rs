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

    /// Clone contained elements into a vector, with the top element first.
    pub fn collect(&self) -> Vec<T> {
        if let Some(mut layer) = self.layers.as_ref() {
            let mut result = vec![layer.top.clone(); self.height];
            for item in result.iter_mut().skip(1) {
                layer = layer.lower.layers.as_ref().unwrap();
                *item = layer.top.clone();
            }
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Pile::<bool>::EMPTY.collect(), vec![]);
    }

    #[test]
    fn one_level() {
        let p1 = Pile::from(true);
        assert_eq!(p1.collect(), vec![true]);
    }

    #[test]
    fn two_levels() {
        let p1 = Pile::from(22);
        {
            let p2 = p1.pile(11);
            assert_eq!(p2.collect(), vec![11, 22]);
        }
        assert_eq!(p1.collect(), vec![22]);
    }
}
