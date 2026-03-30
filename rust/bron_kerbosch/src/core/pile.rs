use std::iter::FusedIterator;

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

    pub fn iter(&'a self) -> PileIterator<'a, T> {
        PileIterator {
            layer: self.layers.as_ref(),
        }
    }
}

pub struct PileIterator<'a, T> {
    layer: Option<&'a Layer<'a, T>>,
}

impl<'a, T> Iterator for PileIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(layer) = self.layer {
            self.layer = layer.lower.layers.as_ref();
            Some(&layer.top)
        } else {
            None
        }
    }
}

impl<'a, T> FusedIterator for PileIterator<'a, T> {}
