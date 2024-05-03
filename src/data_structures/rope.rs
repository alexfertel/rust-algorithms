use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

/// A Rope is a data structure designed for efficient manipulation of large strings
/// of text by dividing the text into smaller segments represented as nodes in a binary tree.
/// Each leaf (end node) holds a string and a length (also known as a "weight"),
/// and each node further up the tree holds the sum of the lengths of all the leaves in its left subtree.
pub enum Rope {
    Leaf(String),
    Node {
        left: Option<Box<Rope>>,
        right: Option<Box<Rope>>,
        weight: usize,
    },
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rope::Leaf(s) => write!(f, "{s}"),
            Rope::Node { left, right, .. } => {
                if let Some(l) = left {
                    write!(f, "{l}")?;
                }
                if let Some(r) = right {
                    write!(f, "{r}")?;
                }

                Ok(())
            }
        }
    }
}

impl Rope {
    pub fn get_weight(&self) -> usize {
        match self {
            Rope::Node { weight, .. } => *weight,
            Rope::Leaf(str) => str.len(),
        }
    }

    pub fn split(self: Box<Rope>, index: usize) -> (Option<Box<Rope>>, Option<Box<Rope>>) {
        if index == 0 {
            return (None, Some(self));
        }

        match *self {
            Rope::Leaf(ref str) => match index.cmp(&(str.len() - 1)) {
                Ordering::Equal | Ordering::Greater => (Some(self), None),
                Ordering::Less => (
                    Some(Box::new(Rope::Leaf(String::from(&str[0..index + 1])))),
                    Some(Box::new(Rope::Leaf(String::from(&str[(index + 1)..])))),
                ),
            },
            Rope::Node {
                left,
                right,
                weight,
            } => match index.cmp(&(weight - 1)) {
                Ordering::Equal => (left, right),
                Ordering::Less => {
                    let Some(left) = left else {
                        panic!("Rope weight is inconsistent with left child");
                    };

                    match left.split(index) {
                        (Some(first), Some(second)) => (
                            Some(first),
                            Some(Box::new(Rope::Node {
                                left: Some(second),
                                right,
                                weight: weight - index - 1,
                            })),
                        ),
                        (Some(left), None) => (Some(left), right),
                        (_, _) => panic!("Invalid split results"),
                    }
                }
                Ordering::Greater => {
                    let Some(right) = right else {
                        panic!("Rope weight is inconsistent with left child");
                    };

                    match right.split(index - weight) {
                        (Some(first), Some(second)) => (
                            Some(Box::new(Rope::Node {
                                weight: {
                                    match left {
                                        Some(ref left) => left.get_weight(),
                                        None => 0,
                                    }
                                },
                                left,
                                right: Some(first),
                            })),
                            Some(second),
                        ),
                        (Some(right), None) => (
                            Some(Box::new(Rope::Node {
                                left,
                                right: Some(right),
                                weight,
                            })),
                            None,
                        ),
                        (_, _) => panic!("Invalid split results"),
                    }
                }
            },
        }
    }

    pub fn concat(self: Box<Rope>, target: Box<Rope>) -> Box<Rope> {
        let weight = self.get_weight();
        Box::new(Rope::Node {
            left: Some(self),
            right: Some(target),
            weight,
        })
    }

    pub fn insert(self: Box<Rope>, value: &str, index: usize) -> Box<Rope> {
        if value.len() == 0 {
            self
        } else {
            let new_leaf = Box::new(Rope::Leaf(String::from(value)));
            match self.split(index) {
                (Some(left), Some(right)) => left.concat(new_leaf.concat(right)),
                (None, Some(rope)) => new_leaf.concat(rope),
                (Some(rope), None) => rope.concat(new_leaf),
                _ => panic!("Split operation produced None, None"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rope;

    #[test]
    fn to_string() {
        let rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 11,
        });
        assert_eq!(rope.to_string(), "hello world");
    }

    #[test]
    fn concats() {
        let left = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
            right: None,
            weight: 6,
        });

        let right = Box::new(Rope::Leaf(String::from("world")));
        let concat = left.concat(right);
        let weight = concat.get_weight();

        assert_eq!(weight, 6);
    }

    #[test]
    fn split_at_length() {
        let rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Node {
                left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
                right: None,
                weight: 6,
            })),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 6,
        });

        let (Some(first), None) = rope.split(("hello world").len() - 1) else {
            panic!("Split at last element should return the original Rope and None")
        };
        assert_eq!(first.get_weight(), 6);
        assert_eq!(first.to_string(), "hello world");
    }

    #[test]
    fn split_in_half() {
        let rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Node {
                left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
                right: None,
                weight: 6,
            })),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 6,
        });

        let (Some(first), Some(second)) = rope.split(("hello world").len() / 2) else {
            panic!("Split in half should return two non-None ropes")
        };
        assert_eq!(first.to_string(), "hello ");
        assert_eq!(second.to_string(), "world");
    }

    #[test]
    fn insert_at_start() {
        let mut rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Node {
                left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
                right: None,
                weight: 6,
            })),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 6,
        });

        rope = rope.insert("He said: ", 0);
        assert_eq!(rope.to_string(), "He said: hello world");
    }

    #[test]
    fn insert_at_end() {
        let mut rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Node {
                left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
                right: None,
                weight: 6,
            })),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 6,
        });

        rope = rope.insert("!!!", ("hello world").len());
        assert_eq!(rope.to_string(), "hello world!!!");
    }

    #[test]
    fn insert_in_middle() {
        let mut rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Node {
                left: Some(Box::new(Rope::Leaf(String::from("foo ")))),
                right: None,
                weight: 4,
            })),
            right: Some(Box::new(Rope::Leaf(String::from("baz")))),
            weight: 4,
        });

        rope = rope.insert("bar ", 3);
        assert_eq!(rope.to_string(), "foo bar baz");
    }
}
