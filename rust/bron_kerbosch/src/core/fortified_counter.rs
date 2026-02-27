use std::marker::PhantomData;

// Counts the coming and going of elements and, in debug build only, checks their identity.
pub struct FortifiedCounter<T> {
    phantom: PhantomData<T>,
    count: usize,
    #[cfg(debug_assertions)]
    individuals: Vec<T>, // set-like, but don't force trait Hash or Ord on T here
}

impl<T> FortifiedCounter<T>
where
    T: Copy + Eq,
{
    fn invariant(&self) {
        #[cfg(debug_assertions)]
        assert_eq!(self.count, self.individuals.len());
    }

    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            count: 0,
            #[cfg(debug_assertions)]
            individuals: vec![],
        }
    }

    pub fn empty(&self) -> bool {
        self.invariant();
        self.count == 0
    }

    pub fn contains(&self, element: &T) -> bool {
        self.occurrences(element) == 1
    }

    pub fn add(&mut self, element: &T) {
        debug_assert_eq!(self.occurrences(element), 0);
        self.invariant();
        self.count += 1;
        #[cfg(debug_assertions)]
        self.individuals.push(*element);
        self.invariant();
    }

    pub fn remove(&mut self, element: &T) {
        debug_assert_eq!(self.occurrences(element), 1);
        self.invariant();
        self.count -= 1;
        #[cfg(debug_assertions)]
        self.individuals.retain(|e| *e != *element);
        self.invariant();
    }

    fn occurrences(&self, element: &T) -> usize {
        #[cfg(not(debug_assertions))]
        {
            _ = element;
            unreachable!("debug_assert only, please");
        }
        #[cfg(debug_assertions)]
        self.individuals.iter().filter(|&e| *e == *element).count()
    }
}
