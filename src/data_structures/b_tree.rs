use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

/// A self-balancing tree data structure.
///
/// A BTree maintains sorted data and allows searches, sequential access, insertions, and deletions in logarithmic time.
/// The B-tree generalizes the binary search tree, allowing for nodes with more than two children.
/// A B-tree of order m is a tree which satisfies the following properties:
/// 1. Every node has at most m children.
/// 2. Every non-leaf node (except the root) has at least m / 2 children.
/// 3. The root has at least two children if it is not a leaf node.
/// 4. A non-leaf node with k children contains k−1 keys.
/// 5. All leaves appear on the same level.
/// 6. A non-leaf node with k children contains k−1 keys.
/// 7. All leaves appear on the same level.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::data_structures::BTree;
///
/// let mut tree = BTree::new(2);
/// tree.insert(10);
/// tree.insert(20);
/// tree.insert(30);
/// tree.insert(5);
///
/// assert!(tree.search(10));
/// assert_eq!(tree.search(15), false);
/// ```
pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

/// BTree properties
///
/// # Reference
///
/// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match _children {
                Some(_children) => _children,
                None => Vec::with_capacity(degree),
            },
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    // Split Child expects the Child Node to be full
    /// Move the middle_key to parent node and split the child_node's
    /// keys/chilren_nodes into half
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first() {
            Some((_first, _others)) => {
                // We don't need _first, as it will move to parent node.
                _others.to_vec()
            }
            None => Vec::with_capacity(self.max_keys),
        };
        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(self.mid_key_index + 1))
        } else {
            None
        };
        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }

        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();
        if node.is_leaf() {
            // Just insert it, as we know this method will be called only when node is not full
            node.keys.insert(u_index, key);
        } else {
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }

            self.insert_non_full(&mut node.children[u_index], key);
        }
    }

    fn traverse_node<T: Ord + Debug>(&self, node: &Node<T>, depth: usize) {
        if node.is_leaf() {
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                self.traverse_node(&node.children[index], _depth);
                // Check https://doc.rust-lang.org/std/fmt/index.html
                // And https://stackoverflow.com/a/35280799/2849127
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            self.traverse_node(node.children.last().unwrap(), _depth);
        }
    }
}

/// BTree implementation
///
impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    /// Create a new BTree with the given branch factor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BTree;
    ///
    /// let mut tree = BTree::new(2);
    ///
    /// assert_eq!(tree.search(10), false);
    /// assert_eq!(tree.search(15), false);
    /// ```
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        BTree {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    /// Insert a key into the BTree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BTree;
    ///
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// tree.insert(5);
    ///
    /// assert!(tree.search(1));
    /// assert_eq!(tree.search(4), false);
    /// ```
    pub fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root) {
            // Create an empty root and split the old root...
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    /// Traverse the BTree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BTree;
    ///
    /// let mut tree = BTree::new(2);
    /// tree.insert(20);
    /// tree.insert(10);
    /// tree.insert(30);
    /// tree.insert(5);
    ///
    /// tree.traverse();
    /// ```
    pub fn traverse(&self) {
        self.props.traverse_node(&self.root, 0);
        println!();
    }

    /// Check if the BTree is empty.
    ///
    /// # Returns
    ///
    /// `true` if the BTree is empty, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BTree;
    ///
    /// let mut tree = BTree::new(2);
    /// assert!(tree.is_empty());
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.keys.is_empty()
    }

    /// Search for a key in the BTree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_algorithms::data_structures::BTree;
    ///
    /// let mut tree = BTree::new(2);
    /// tree.insert(1);
    /// tree.insert(5);
    ///
    /// assert!(tree.search(1));
    /// assert_eq!(tree.search(15), false);
    /// ```
    pub fn search(&self, key: T) -> bool {
        let mut current_node = &self.root;
        let mut index: isize;
        loop {
            index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
            while index >= 0 && current_node.keys[index as usize] > key {
                index -= 1;
            }

            let u_index: usize = usize::try_from(index + 1).ok().unwrap();
            if index >= 0 && current_node.keys[u_index - 1] == key {
                break true;
            } else if current_node.is_leaf() {
                break false;
            } else {
                current_node = &current_node.children[u_index];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn test_search() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(11);
        tree.insert(12);
        tree.insert(15);
        assert!(tree.search(15));
        assert_eq!(tree.search(16), false);
    }
}
