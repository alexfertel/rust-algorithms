use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::{Index, Range},
};

pub struct NodeData {
    pub left: Option<Box<Rope>>,
    pub right: Option<Box<Rope>>,
    pub weight: usize,
}

impl Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(l) = &self.left {
            write!(f, "{l}")?;
        }
        if let Some(r) = &self.right {
            write!(f, "{r}")?;
        }

        Ok(())
    }
}

/// A Rope is a data structure designed for efficient manipulation of large strings
/// of text by dividing the text into smaller segments represented as nodes in a binary tree.
/// Each leaf (end node) holds a string and a length (also known as a "weight"),
/// and each node further up the tree holds the sum of the lengths of all the leaves in its left subtree.
pub enum Rope {
    Leaf(String),
    Node(NodeData),
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rope::Leaf(s) => write!(f, "{s}"),
            Rope::Node(n) => write!(f, "{n}"),
        }
    }
}

impl Index<usize> for Rope {
    type Output = str;

    fn index(&self, index: usize) -> &str {
        match self {
            Rope::Leaf(data) => return &data[index..index + 1],
            Rope::Node(NodeData {
                left,
                right,
                weight,
            }) => {
                if index < *weight {
                    if let Some(left) = left {
                        return &left[index];
                    } else {
                        unreachable!("Rope weight is inconsistent with left child");
                    }
                } else {
                    if let Some(right) = right {
                        return &right[index - weight];
                    } else {
                        unreachable!("Rope weight is inconsistent with right child");
                    }
                }
            }
        }
    }
}

impl Rope {
    pub fn get_weight(&self) -> usize {
        match self {
            Rope::Node(node) => node.weight,
            Rope::Leaf(str) => str.len(),
        }
    }

    pub fn split_at(self: Box<Rope>, index: usize) -> (Option<Box<Rope>>, Option<Box<Rope>>) {
        if index == 0 {
            return (None, Some(self));
        }

        match *self {
            Rope::Leaf(ref str) => {
                if index >= (str.len() - 1) {
                    (Some(self), None)
                } else if index == 0 {
                    (None, Some(self))
                } else {
                    (
                        Some(Box::new(Rope::Leaf(String::from(&str[0..index])))),
                        Some(Box::new(Rope::Leaf(String::from(&str[index..])))),
                    )
                }
            }
            Rope::Node(node) => match index.cmp(&(node.weight - 1)) {
                Ordering::Equal => (node.left, node.right),
                Ordering::Less => {
                    let Some(left) = node.left else {
                        unreachable!("Rope weight is inconsistent with left child");
                    };

                    let (left, right) = left.split_at(index);
                    if left.is_none() && right.is_none() {
                        unreachable!("Invalid split results");
                    }

                    if right.is_some() {
                        let new_right = NodeData {
                            left: right,
                            right: node.right,
                            weight: node.weight - index - 1,
                        };
                        let new_right = Some(Box::new(Rope::Node(new_right)));
                        return (left, new_right);
                    };

                    (left, node.right)
                }
                Ordering::Greater => {
                    let Some(right) = node.right else {
                        unreachable!("Rope weight is inconsistent with left child");
                    };

                    let (left, right) = right.split_at(index - node.weight);
                    if left.is_none() && right.is_none() {
                        unreachable!("Invalid split results");
                    }

                    if left.is_some() && right.is_some() {
                        let weight = node.left.as_ref().map_or(0, |l| l.get_weight());
                        let new_left = NodeData {
                            weight,
                            right: left,
                            left: node.left,
                        };
                        let new_left = Some(Box::new(Rope::Node(new_left)));
                        (new_left, right)
                    } else if left.is_some() {
                        let new_left = NodeData {
                            left: node.left,
                            right: left,
                            weight: node.weight,
                        };
                        let new_left = Some(Box::new(Rope::Node(new_left)));
                        (new_left, None)
                    } else {
                        (node.left, right)
                    }
                }
            },
        }
    }

    pub fn concat(self: Box<Rope>, target: Box<Rope>) -> Box<Rope> {
        Box::new(Rope::Node(NodeData {
            weight: self.get_weight(),
            right: Some(target),
            left: Some(self),
        }))
    }

    pub fn insert(self: Box<Rope>, value: &str, index: usize) -> Box<Rope> {
        if value.len() == 0 {
            return self;
        }

        let new_leaf = Box::new(Rope::Leaf(String::from(value)));
        match self.split_at(index) {
            (Some(left), Some(right)) => left.concat(new_leaf.concat(right)),
            (None, Some(rope)) => new_leaf.concat(rope),
            (Some(rope), None) => rope.concat(new_leaf),
            _ => unreachable!("Split operation produced None, None"),
        }
    }

    pub fn delete_at(
        self: Box<Rope>,
        start: usize,
        length: usize,
    ) -> Result<Box<Rope>, &'static str> {
        if length <= 0 {
            return Err("Length to remove must be positive");
        }
        if start + length - 1 >= self.to_string().len() {
            return Err("Index out of range");
        }

        let (left, remain) = self.split_at(start);
        let Some(remain) = remain else {
            return Ok(left.unwrap_or_else(|| unreachable!("Split operation produced None, None")));
        };

        let (_, right) = remain.split_at(length);
        if left.is_some() {
            return Ok(Box::new(Rope::Node(NodeData {
                weight: left.as_ref().unwrap().get_weight(),
                left,
                right,
            })));
        } else if right.is_some() {
            Ok(right.unwrap())
        } else {
            Err("Cannot delete the entire input")
        }
    }

    pub fn slice(&self, start: usize, size: usize) -> String {
        self.to_string()[start..start + size].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeData, Rope};

    macro_rules! boxed {
        ($wrapped: expr) => {
            Box::new($wrapped)
        };
    }

    macro_rules! leaf {
        ($wrapped: expr) => {
            Some(boxed!(Rope::Leaf($wrapped)))
        };
    }

    macro_rules! node {
        (left: $left: expr, right: $right: expr, weight: $weight: expr $(,)?) => {
            boxed!(Rope::Node(NodeData {
                left: $left,
                right: $right,
                weight: $weight
            }))
        };
    }

    #[test]
    fn to_string() {
        let rope = node! {
            left: leaf!(String::from("hello ")),
            right: leaf!(String::from("world")),
            weight: 6,
        };
        assert_eq!(rope.to_string(), "hello world");
    }

    #[test]
    fn concats() {
        let left = node! {
            left: leaf!(String::from("hello ")),
            right: None,
            weight: 6,
        };

        let right = boxed!(Rope::Leaf(String::from("world")));
        let concat = left.concat(right);
        let weight = concat.get_weight();

        assert_eq!(weight, 6);
    }

    #[test]
    fn split_at_start() {
        let rope = node! {
        left: Some(node! {
            left: leaf!(String::from("hello ")),
            right: None,
            weight: 6,
        }),
        right: leaf!(String::from("world")),
        weight: 6,
        };

        let (None, Some(first)) = rope.split_at(0) else {
            panic!("Split at first element should return the original Rope and None")
        };
        assert_eq!(first.get_weight(), 6);
        assert_eq!(first.to_string(), "hello world");
    }

    #[test]
    fn split_at_length() {
        let rope = node! {
            left: Some(node! {
                left: leaf!(String::from("hello ")),
                right: None,
                weight: 6,
            }),
            right: leaf!(String::from("world")),
            weight: 6,
        };

        let (Some(first), None) = rope.split_at(("hello world").len()) else {
            panic!("Split at last element should return the original Rope and None")
        };
        assert_eq!(first.get_weight(), 6);
        assert_eq!(first.to_string(), "hello world");
    }

    #[test]
    fn split_in_half() {
        let rope = node! {
            left: Some(node! {
                left: leaf!(String::from("hello ")),
                right: None,
                weight: 6,
            }),
            right: leaf!(String::from("world")),
            weight: 6,
        };

        let (Some(first), Some(second)) = rope.split_at(("hello world").len() / 2) else {
            panic!("Split in half should return two non-None ropes")
        };
        assert_eq!(first.to_string(), "hello ");
        assert_eq!(second.to_string(), "world");
    }

    #[test]
    fn split_at_leaf() {
        let rope = node! {
            left: leaf!(String::from("hello!")),
            right: None,
            weight: 6,
        };

        let (left, right) = rope.split_at(2);
        if left.is_none() || right.is_none() {
            panic!("Invalid split results: left & right must be Some(_)");
        }
        assert_eq!(left.unwrap().to_string(), "he");
        assert_eq!(right.unwrap().to_string(), "llo!");
    }

    #[test]
    fn insert_at_start() {
        let mut rope = node! {
            left: Some(node! {
                left: leaf!(String::from("hello ")),
                right: None,
                weight: 6,
            }),
            right: leaf!(String::from("world")),
            weight: 6,
        };

        rope = rope.insert("He said: ", 0);
        assert_eq!(rope.to_string(), "He said: hello world");
    }

    #[test]
    fn insert_at_end() {
        let mut rope = node! {
            left: Some(node! {
                left: leaf!(String::from("hello ")),
                right: None,
                weight: 6,
            }),
            right: leaf!(String::from("world")),
            weight: 6,
        };

        rope = rope.insert("!!!", ("hello world").len());
        assert_eq!(rope.to_string(), "hello world!!!");
    }

    #[test]
    fn insert_in_middle() {
        let mut rope = node! {
            left: Some(node! {
                left: leaf!(String::from("foo ")),
                right: None,
                weight: 4,
            }),
            right: leaf!(String::from("baz")),
            weight: 4,
        };

        rope = rope.insert("bar ", 3);
        assert_eq!(rope.to_string(), "foo bar baz");
    }

    #[test]
    fn delete_in_middle() {
        let rope = node! {
            left: Some(node! {
                left: leaf!(String::from("foo ")),
                right: None,
                weight: 4,
            }),
            right: leaf!(String::from("baz")),
            weight: 4,
        };

        let rope = rope.delete_at(4, 3).unwrap();
        assert_eq!(rope.to_string(), "foo ")
    }

    #[test]
    fn delete_at_start() {
        let rope = node! {
            left: Some(node! {
                left: leaf!(String::from("foo ")),
                right: None,
                weight: 4,
            }),
            right: leaf!(String::from("baz")),
            weight: 4,
        };
        let rope = rope.delete_at(0, 4).unwrap();
        assert_eq!(rope.to_string(), "baz")
    }

    #[test]
    fn indexing() {
        let rope = node! {
            left: leaf!(String::from("hello ")),
            right: leaf!(String::from("world")),
            weight: 6,
        };
        assert_eq!(&rope[0], "h");
        assert_eq!(&rope[1], "e");
        assert_eq!(&rope[2], "l");
        assert_eq!(&rope[3], "l");
        assert_eq!(&rope[4], "o");
        assert_eq!(&rope[5], " ");
        assert_eq!(&rope[6], "w");
        assert_eq!(&rope[7], "o");
        assert_eq!(&rope[8], "r");
        assert_eq!(&rope[9], "l");
        assert_eq!(&rope[10], "d");
    }

    #[test]
    fn slicing() {
        let rope = node! {
            left: leaf!(String::from("hello ")),
            right: leaf!(String::from("world")),
            weight: 6,
        };
        assert_eq!(rope.slice(0, 5), "hello");
        assert_eq!(rope.slice(5, 6), " world");
    }
}
