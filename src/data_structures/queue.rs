pub struct Queue<T> {
    vec: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { vec: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        self.vec.push(item);
        true
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.vec[0])
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.vec.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn fill_and_empty() {
        let mut q = Queue::new();

        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek(), None);

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert!(!q.is_empty());
        assert_eq!(q.len(), 3);
        assert_eq!(q.peek(), Some(&1));

        q.dequeue();
        q.dequeue();
        q.dequeue();

        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek(), None);
    }
}
