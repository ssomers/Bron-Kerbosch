use super::priority_queue::*;

#[test]
fn empty() {
    let mut q: PriorityQueue<bool> = PriorityQueue::new(0);
    assert!(q.pop().is_none());
}

#[test]
fn one_level_single() {
    let mut q = PriorityQueue::new(1);
    q.put(Priority::new(1).unwrap(), ());
    assert_eq!(q.pop(), Some(()));
    assert_eq!(q.pop(), None);
}

#[test]
fn one_level_double() {
    let mut q = PriorityQueue::new(1);
    q.put(Priority::new(1).unwrap(), ());
    q.put(Priority::new(1).unwrap(), ());
    assert_eq!(q.pop(), Some(()));
    assert_eq!(q.pop(), Some(()));
    assert_eq!(q.pop(), None);
}

#[test]
fn two_level_single() {
    let mut q = PriorityQueue::new(2);
    q.put(Priority::new(2).unwrap(), ());
    assert_eq!(q.pop(), Some(()));
    assert_eq!(q.pop(), None);
}

#[test]
fn two_level_ascending() {
    let mut q = PriorityQueue::new(2);
    q.put(Priority::new(2).unwrap(), 22);
    q.put(Priority::new(1).unwrap(), 11);
    assert_eq!(q.pop(), Some(11));
    assert_eq!(q.pop(), Some(22));
    assert_eq!(q.pop(), None);
}

#[test]
fn two_level_descending() {
    let mut q = PriorityQueue::new(2);
    q.put(Priority::new(1).unwrap(), 22);
    q.put(Priority::new(2).unwrap(), 11);
    assert_eq!(q.pop(), Some(22));
    assert_eq!(q.pop(), Some(11));
    assert_eq!(q.pop(), None);
}
