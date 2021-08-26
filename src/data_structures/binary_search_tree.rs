type Node<T> = Option<T>;

#[derive(Debug, PartialEq, Eq)]
pub struct BinarySearchTree<T> {
    value: T,
    left: Node<T>,
    right: Node<T>,
}

impl<T> BinarySearchTree<T> {
    pub fn new(value: T) -> Self {
        BinarySearchTree {
            value,
            right: None,
            left: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn creates_with_new() {
        let expected = BinarySearchTree {
            value: 1,
            right: None,
            left: None,
        };

        let actual = BinarySearchTree::new(1);
        assert_eq!(expected, actual);
    }
}
