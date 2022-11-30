// a vector-based implementation of the stack data type
pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    // a constructor that returns an instance of Stack<T> backed by an empty Vec<T>
    pub fn new() -> Self {
        Stack { vec: Vec::new() }
    }

    // mutates the stack by adding an item of type T and returns true
    pub fn push(&mut self, item: T) -> bool {
        self.vec.push(item);
        true
    }

    // returns the number of elements in the stack
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    // returns true if stack is empty else false
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    // returns a Some<&T> if stack is non-empty else None
    // &T is an immutable reference to an element of type T
    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }

    // mutates the stack by removing and returning a Some<T>
    // if stack is non-empty else None
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn starts_empty() {
        let mut q: Stack<String> = Stack::new();

        assert!(q.is_empty());
        assert_eq!(q.pop(), None);
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn fills_and_empties() {
        let mut q = Stack::new();

        q.push(1);
        q.push(2);
        q.push(3);

        assert!(!q.is_empty());
        assert_eq!(q.len(), 3);
        assert_eq!(q.peek(), Some(&3));

        q.pop();
        q.pop();
        q.pop();

        assert!(q.is_empty());
        assert_eq!(q.pop(), None);
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn alternate_mutations() {
        let mut q = Stack::new();

        q.push("A");

        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek(), Some(&"A"));

        q.push("B");

        assert!(!q.is_empty());
        assert_eq!(q.len(), 2);
        assert_eq!(q.peek(), Some(&"B"));

        q.pop();

        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek(), Some(&"A"));

        q.push("C");
        q.push("D");

        assert!(!q.is_empty());
        assert_eq!(q.len(), 3);
        assert_eq!(q.peek(), Some(&"D"));

        q.pop();
        q.pop();

        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek(), Some(&"A"));
    }
}
