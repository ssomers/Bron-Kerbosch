pub type Priority = std::num::NonZero<usize>; // = degree - #of yielded neighbours

pub struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
    lowest_populated_index: usize,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    pub fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority],
            lowest_populated_index: max_priority,
        }
    }

    // Adds an entry, regardless of whether the same element was already added earlier.
    pub fn put(&mut self, priority: Priority, element: T) {
        let index = priority.get() - 1;
        self.stack_per_priority[index].push(element);
        self.lowest_populated_index = self.lowest_populated_index.min(index);
    }

    pub fn pop(&mut self) -> Option<T> {
        while self.lowest_populated_index < self.stack_per_priority.len() {
            if let Some(element) = self.stack_per_priority[self.lowest_populated_index].pop() {
                return Some(element);
            }
            self.lowest_populated_index += 1;
        }
        None
    }

    pub fn contains(&self, priority: Priority, element: T) -> bool {
        if cfg!(debug_assertions) {
            self.stack_per_priority[priority.get() - 1].contains(&element)
        } else {
            panic!("not suitable for use in release code")
        }
    }
}
