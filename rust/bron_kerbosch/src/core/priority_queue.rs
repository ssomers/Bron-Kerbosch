pub type Priority = std::num::NonZero<usize>; // = degree - #of yielded neighbours

pub struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    pub fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority],
        }
    }

    // Adds an entry, regardless of whether the same element was already added earlier.
    pub fn put(&mut self, priority: Priority, element: T) {
        self.stack_per_priority[priority.get() - 1].push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        for stack in &mut self.stack_per_priority {
            if let Some(element) = stack.pop() {
                return Some(element);
            }
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
