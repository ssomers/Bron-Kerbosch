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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue_empty() {
        let mut q: PriorityQueue<bool> = PriorityQueue::new(0);
        assert!(q.pop().is_none());
    }

    #[test]
    fn test_priority_queue_one() {
        let mut q = PriorityQueue::new(1);
        q.put(Priority::new(1).unwrap(), true);
        assert_eq!(q.pop(), Some(true));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn test_priority_queue_two_down() {
        let mut q = PriorityQueue::new(2);
        q.put(Priority::new(2).unwrap(), 22);
        q.put(Priority::new(1).unwrap(), 11);
        assert_eq!(q.pop(), Some(11));
        assert_eq!(q.pop(), Some(22));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn test_priority_queue_two_up() {
        let mut q = PriorityQueue::new(2);
        q.put(Priority::new(1).unwrap(), 22);
        q.put(Priority::new(2).unwrap(), 11);
        assert_eq!(q.pop(), Some(22));
        assert_eq!(q.pop(), Some(11));
        assert_eq!(q.pop(), None);
    }
}
