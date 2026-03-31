use std::marker::PhantomData;
use std::ops::Not;

// Counts the coming and going of elements and, in debug build only, checks their identity.
pub struct FortifiedCounter<T> {
    phantom: PhantomData<T>,
    counter: usize,
    #[cfg(debug_assertions)]
    individuals: Vec<T>, // set-like, but don't force trait Hash or Ord on T here
}

impl<T> FortifiedCounter<T>
where
    T: Copy + Eq,
{
    fn invariant(&self) {
        #[cfg(debug_assertions)]
        assert_eq!(self.counter, self.individuals.len());
    }

    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            counter: 0,
            #[cfg(debug_assertions)]
            individuals: vec![],
        }
    }

    pub fn count(&self) -> usize {
        self.invariant();
        self.counter
    }

    pub fn contains(&self, element: T) -> bool {
        #[cfg(not(debug_assertions))]
        {
            _ = element;
            unreachable!("debug_assert only, please");
        }
        #[cfg(debug_assertions)]
        match self.individuals.iter().filter(|&e| *e == element).count() {
            0 => false,
            1 => true,
            _ => unreachable!("duplicate individual"),
        }
    }

    pub fn add(&mut self, element: T) {
        debug_assert!(self.contains(element).not());
        self.invariant();
        self.counter += 1;
        #[cfg(debug_assertions)]
        self.individuals.push(element);
        self.invariant();
    }

    pub fn remove(&mut self, element: T) {
        debug_assert!(self.contains(element));
        self.invariant();
        self.counter -= 1;
        #[cfg(debug_assertions)]
        self.individuals.retain(|e| *e != element);
        self.invariant();
    }
}
