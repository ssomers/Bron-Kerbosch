/// Cheap & simple immutable stack data structure, of which the layers
/// somehow live long enough (e.g. they are on the runtime stack).
/// The only supported change is to add, and the only supported query is to
/// convert to a vector.
pub struct Pile<'a, T> {
    top: T,
    earlier: Option<&'a Pile<'a, T>>,
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    fn height(&self) -> usize {
        match self.earlier {
            Some(earlier) => earlier.height() + 1,
            None => 1,
        }
    }

    fn push_to(&self, result: &mut Vec<T>) {
        if let Some(earlier) = self.earlier {
            earlier.push_to(result);
        }
        result.push(self.top.clone());
    }

    /// Create a pile containing one element
    pub fn from(t: T) -> Self {
        Pile::on(None, t)
    }

    /// Create a pile optionally on top of an existing pile
    pub fn on(earlier: Option<&'a Pile<'a, T>>, top: T) -> Self {
        Self { earlier, top }
    }

    /// Clone contained elements into a vector, in the order they were placed
    pub fn collect(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.height());
        self.push_to(&mut result);
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
            let p2 = Pile::on(Some(&p1), 2);
            assert_eq!(p2.collect(), vec![4, 2]);
        }
        assert_eq!(p1.collect(), vec![4]);
    }
}
