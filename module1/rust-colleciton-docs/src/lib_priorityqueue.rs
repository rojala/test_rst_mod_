use std::collections::BinaryHeap;

/// A priority queue that sorts items by priority using a BinaryHeap.
#[derive(Debug)]
pub struct PriorityQueue<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    /// Creates a new, empty PriorityQueue.
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    /// Inserts an item into the priority queue.
    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    /// Removes and returns the item with the highest priority, or None if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    /// Returns a reference to the item with the highest priority, or None if empty.
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the number of items in the queue.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::PriorityQueue;

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        pq.push(3);
        pq.push(5);
        pq.push(1);

        assert_eq!(pq.len(), 3);

        assert_eq!(pq.peek(), Some(&5));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), Some(1));
        assert!(pq.is_empty());
    }
}
