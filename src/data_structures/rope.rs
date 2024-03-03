use std::cmp::Ordering;

// A Rope is a data structure designed for efficient manipulation of large strings
// of text by dividing the text into smaller segments represented as nodes in a binary tree.
// Each leaf (end node) holds a string and a length (also known as a "weight"),
// and each node further up the tree holds the sum of the lengths of all the leaves in its left subtree.
pub enum Rope {
    Leaf(String),
    Node {
        left: Option<Box<Rope>>,
        right: Option<Box<Rope>>,
        weight: usize,
    },
}

impl Rope {
    pub fn get_weight(&self) -> usize {
        match &*self {
            Rope::Node {
                left: _,
                right: _,
                weight,
            } => *weight,
            Rope::Leaf(str) => str.len(),
        }
    }

    pub fn to_string(&self) -> String {
        let leaves: Vec<&Rope> = self.collect_leaves();
        let mut result = String::new();
        for leaf in leaves {
            if let Rope::Leaf(value) = &*leaf {
                result.push_str(&*value)
            }
        }
        result
    }

    pub fn collect_leaves(&self) -> Vec<&Rope> {
        if let Rope::Leaf(_) = *self {
            return vec![self];
        }

        let mut result = Vec::new();
        if let Rope::Node {
            left,
            right,
            weight: _,
        } = &*self
        {
            if let Some(left) = left {
                let mut left_leaves = left.collect_leaves();
                result.append(&mut left_leaves);
            }

            if let Some(right) = right {
                let mut right_leaves = right.collect_leaves();
                result.append(&mut right_leaves);
            }
        }

        result
    }

    pub fn split(self: Box<Rope>, index: usize) -> (Option<Box<Rope>>, Option<Box<Rope>>) {
        if index == 0 {
            return (None, Some(self));
        }

        match *self {
            Rope::Leaf(str) => match index.cmp(&(str.len() - 1)) {
                Ordering::Equal => (Some(Box::new(Rope::Leaf(str))), None),
                Ordering::Less => (
                    Some(Box::new(Rope::Leaf(String::from(&str[0..index + 1])))),
                    Some(Box::new(Rope::Leaf(String::from(&str[(index + 1)..])))),
                ),
                Ordering::Greater => (Some(Box::new(Rope::Leaf(str))), None),
            },
            Rope::Node {
                left,
                right,
                weight,
            } => match index.cmp(&(weight - 1)) {
                Ordering::Equal => (left, right),
                Ordering::Less => {
                    if let Some(left) = left {
                        return match left.split(index) {
                            (Some(first), Some(second)) => (
                                Some(first),
                                Some(Box::new(Rope::Node {
                                    left: Some(second),
                                    right: right,
                                    weight: weight - index - 1,
                                })),
                            ),
                            (Some(left), None) => (Some(left), right),
                            (_, _) => panic!("Invalid split results"),
                        };
                    } else {
                        panic!("Rope weight is inconsistent with left child");
                    }
                }
                Ordering::Greater => {
                    if let Some(right) = right {
                        return match right.split(index - weight) {
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
                        };
                    } else {
                        panic!("Rope weight is inconsistent with left child");
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
    fn collect_as_string() {
        let rope = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
            right: Some(Box::new(Rope::Leaf(String::from("world")))),
            weight: 11,
        });
        assert_eq!(rope.to_string(), "hello world");
    }

    #[test]
    fn concat() {
        let left = Box::new(Rope::Node {
            left: Some(Box::new(Rope::Leaf(String::from("hello ")))),
            right: None,
            weight: 6,
        });

        let right = Box::new(Rope::Leaf(String::from("world")));
        let concat = left.concat(right);
        let Rope::Node {
            left: _,
            right: _,
            weight,
        } = *concat
        else {
            panic!("")
        };

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
