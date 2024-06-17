use std::cmp::Ordering;
use std::ops::Deref;

/// A binary search tree (BST) is a binary tree where each node has at most two children, and the
/// left child is less than the parent, and the right child is greater than the parent. This
/// implementation is a simple BST, and does not have any balancing mechanisms.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::data_structures::BinarySearchTree;
///
/// let mut tree = BinarySearchTree::new();
/// tree.insert(5);
/// tree.insert(3);
/// tree.insert(7);
/// tree.insert(1);
/// tree.insert(4);
/// tree.insert(6);
/// tree.insert(8);
///
/// assert!(tree.search(&5));
/// assert!(tree.search(&3));
/// assert!(tree.search(&7));
/// assert!(tree.search(&1));
/// assert!(tree.search(&4));
/// assert!(tree.search(&6));
/// assert!(tree.search(&8));
/// assert!(!tree.search(&0));
/// assert!(!tree.search(&2));
/// assert!(!tree.search(&9));
/// ```
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

/// Default implementation for BinarySearchTree
///
/// Creates a new empty BinarySearchTree
impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    /// Create a new, empty `BinarySearchTree`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let tree: BinarySearchTree<i32> = BinarySearchTree::default();
    ///
    /// assert!(tree.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// Create a new, empty `BinarySearchTree`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let tree: BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert!(tree.is_empty());
    /// ```
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Determines if this tree is empty.
    ///
    /// # Returns
    ///
    /// `true`` if this tree is empty, and `false`` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// assert!(tree.is_empty());
    /// tree.insert(5);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Find a value in this tree.
    ///
    /// # Returns
    ///
    /// `true`` if the value is in this tree, and `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for in this tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    ///
    /// assert!(tree.search(&5));
    /// assert!(tree.search(&3));
    /// assert!(!tree.search(&0));
    /// assert!(!tree.search(&4));
    /// ```
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        // key == value
                        true
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    /// Creates an iterator which iterates over this tree in ascending order
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// let mut iter = tree.iter();
    ///
    /// assert_eq!(iter.next().unwrap(), &3);
    /// assert_eq!(iter.next().unwrap(), &5);
    /// assert_eq!(iter.next().unwrap(), &7);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    /// Inserts a value into the appropriate location in this tree.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to insert into this tree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// assert!(tree.search(&5));
    /// assert!(tree.search(&3));
    /// assert!(tree.search(&7));
    /// assert!(!tree.search(&0));
    /// assert!(!tree.search(&4));
    /// ```
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }

    /// Gets the smallest value in this tree.
    ///
    /// # Returns
    ///
    /// The smallest value in this tree, or `None` if this tree is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// assert!(tree.minimum().is_none());
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// assert_eq!(*tree.minimum().unwrap(), 3);
    /// ```
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    /// Gets the largest value in this tree.
    ///
    /// # Returns
    ///
    /// The largest value in this tree, or `None` if this tree is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// assert!(tree.maximum().is_none());
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// assert_eq!(*tree.maximum().unwrap(), 7);
    /// ```
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    /// Gets the largest value in this tree smaller than value
    ///
    /// # Arguments
    ///
    /// * `value` - The floor that limits the maximum value returned.
    ///
    /// # Returns
    ///
    /// The largest value in this tree smaller than value, or `None` if this tree is empty
    /// or `value` is smaller than any contained value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// assert_eq!(*tree.floor(&5).unwrap(), 5);
    /// assert_eq!(*tree.floor(&4).unwrap(), 3);
    /// assert_eq!(*tree.floor(&8).unwrap(), 7);
    ///
    /// assert_eq!(tree.floor(&0), None);
    /// ```
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }

    /// Gets the smallest value in this tree larger than value.
    ///
    /// # Arguments
    ///
    /// * `value` - The ceil that limits the minimum value returned.
    ///
    /// # Returns
    ///
    /// The smallest value in this tree larger than value, or `None` if this tree is empty
    /// or `value` is larger than any contained value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    ///
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// assert_eq!(*tree.ceil(&5).unwrap(), 5);
    /// assert_eq!(*tree.ceil(&4).unwrap(), 5);
    /// assert_eq!(*tree.ceil(&0).unwrap(), 3);
    ///
    /// assert_eq!(tree.ceil(&8), None);
    /// ```
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => {
                        // key == value
                        Some(key)
                    }
                }
            }
            None => None,
        }
    }
}

/// Iterator for BinarySearchTree
///
/// Iterates over the tree in ascending order
struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

/// Iterator implementation for BinarySearchTree
///
/// Iterates over the tree in ascending order
impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    /// Get the next value in the tree
    ///
    /// # Returns
    ///
    /// The next value in the tree, or `None` if the iterator is exhausted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BinarySearchTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(7);
    ///
    /// let mut iter = tree.iter();
    ///
    /// assert_eq!(iter.next().unwrap(), &3);
    /// assert_eq!(iter.next().unwrap(), &5);
    /// assert_eq!(iter.next().unwrap(), &7);
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .floor(&"these are not the droids you're looking for")
                .unwrap(),
            "kill him"
        );
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.floor(&"but i was going to tasche station").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(
            *tree.floor(&"you underestimate my power").unwrap(),
            "you fool"
        );
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(
            *tree
                .ceil(&"these are not the droids you're looking for")
                .unwrap(),
            "you are a bold one"
        );
        assert_eq!(
            *tree.ceil(&"another death star").unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(
            *tree.ceil(&"but i was going to tasche station").unwrap(),
            "general kenobi"
        );
        assert_eq!(
            *tree.ceil(&"you underestimate my power").unwrap(),
            "your move"
        );
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
