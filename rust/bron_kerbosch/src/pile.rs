/// Cheap & simple immutable stack data structure, of which the layers
/// somehow live long enough (e.g. they are on the runtime stack).
/// The only supported change is to add, and the only supported query is to
/// convert to a vector.
pub struct Pile<'a, T> {
    top: T,
    height: u32,
    lower: Option<&'a Pile<'a, T>>,
}

impl<'a, T> Pile<'a, T>
where
    T: Clone,
{
    fn push_to(&self, result: &mut Vec<T>) {
        if let Some(lower) = self.lower {
            lower.push_to(result);
        }
        result.push(self.top.clone());
    }

    /// Create a pile containing one element
    pub fn from(t: T) -> Self {
        Pile::on(None, t)
    }

    /// Create a pile optionally on top of an existing pile
    pub fn on(lower: Option<&'a Pile<'a, T>>, top: T) -> Self {
        let height = 1 + lower.map_or(0, |pile| pile.height);
        Self { top, height, lower }
    }

    /// Clone contained elements into a vector, in the order they were placed
    pub fn collect(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(self.height as usize);
        self.push_to(&mut result);
        debug_assert_eq!(result.len(), self.height as usize);
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
