use std::cell::{Ref, RefCell};
use std::iter::Iterator;
use std::rc::Rc;

type LinkToNode<T> = Rc<RefCell<LinkedNode<T>>>;

fn newLinkToNode<T>(value: T) -> LinkToNode<T> {
    let new_node = Rc::new(RefCell::new(LinkedNode::new(value)));
    new_node
}

struct LinkedNode<T> {
    value: T,
    next: Option<LinkToNode<T>>,
}

impl<T> LinkedNode<T> {
    fn new(value: T) -> Self {
        LinkedNode {
            value: value,
            next: None,
        }
    }
}

pub enum LinkedListError {
    EmptyList,
}

pub struct LinkedList<T> {
    start: Option<LinkToNode<T>>,
    tail: Option<LinkToNode<T>>,
    count: u32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            start: None,
            tail: None,
            count: 0,
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }
    // TODO - Use pattern matching.
    pub fn add_last(&mut self, value: T) {
        if self.count == 0 {
            self.count += 1;
            let new_node = newLinkToNode(value);
            self.start = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            return;
        }
        self.count += 1;
        let new_node = newLinkToNode(value);
        (*self.tail.take().unwrap()).borrow_mut().next = Some(Rc::clone(&new_node));
        self.tail = Some(Rc::clone(&new_node));
    }

    pub fn pop_first(&mut self) -> Result<T, LinkedListError> {
        match self.start.take() {
            Some(pointer) => {
                match (*pointer).borrow_mut().next.take() {
                    Some(next) => {
                        self.count -= 1;
                        self.start = Some(Rc::clone(&next));
                    }
                    None => {
                        self.count = 0;
                        self.tail.take();
                    }
                }; // TODO - Create an error for this.
                Ok(Rc::try_unwrap(pointer).ok().unwrap().into_inner().value)
            }
            None => Err(LinkedListError::EmptyList),
        }
    }
    pub fn add_first(&mut self, value: T) {
        let new_node = newLinkToNode(value);
        self.count += 1;
        match self.start.take() {
            Some(pointer) => {
                (*new_node).borrow_mut().next = Some(pointer);
                self.start = Some(Rc::clone(&new_node));
            }
            None => {
                self.start = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }
    pub fn pop_last(&mut self) -> Result<T, LinkedListError> {
        if self.count == 0 {
            return Err(LinkedListError::EmptyList);
        }
        self.count -= 1;
        if self.count == 0 {
            self.tail.take();
            return Ok(Rc::try_unwrap(self.start.take().unwrap())
                .ok()
                .unwrap()
                .into_inner()
                .value);
        };
        // TODO - try to do this without use a rc pointer for iterate, maybe
        // use ref
        let mut pointer = self.start.take().unwrap();
        self.start = Some(Rc::clone(&pointer));

        for _ in 1..self.count {
            let temp = match &((*pointer).borrow().next) {
                Some(value) => Rc::clone(value),
                None => {
                    break;
                }
            };
            pointer = temp;
        }
        let result = (*pointer).borrow_mut().next.take().unwrap();
        self.tail = Some(pointer);

        Ok(Rc::try_unwrap(result).ok().unwrap().into_inner().value)
    }
    pub fn peek_first(&self) -> Option<Ref<T>> {
        self.start
            .as_ref()
            .map(|node| Ref::map(node.as_ref().borrow(), |node| &(node.value)))
    }

    pub fn peek_last(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.as_ref().borrow(), |node| &(node.value)))
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.start.take() {
            None => None,
            Some(first) => {
                match &((*first).borrow().next) {
                    None => {
                        self.count = 0;
                        self.tail.take();
                    }
                    Some(_) => {
                        self.count -= 1;
                    }
                }
                // TODO - catch the possibles errors
                let mut unwrapped_first = Rc::try_unwrap(first).ok().unwrap().into_inner();
                self.start = unwrapped_first.next.take();
                Some(unwrapped_first.value)
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<A: IntoIterator<Item = T>>(&mut self, iter: A) {
        for element in iter {
            self.add_last(element);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{cmp::Ordering, vec};

    use super::*;

    #[test]
    fn add_last_and_count_test() {
        let mut list = LinkedList::<i32>::new();

        assert_eq!((&list).count(), 0);
        list.add_last(3);
        assert_eq!((&list).count(), 1);
        list.add_last(4);
        assert_eq!((&list).count(), 2);
        list.add_last(2);
        assert_eq!((&list).count(), 3);
    }

    #[test]
    fn iterator_test() {
        let vector = vec![2, 5, 6, 3, 7];

        let mut list = LinkedList::<i32>::new();
        for element in &vector {
            list.add_last(*element);
        }

        let mut index = 0;
        for element in list {
            assert_eq!(element, vector[index]);
            index += 1;
        }
        assert_eq!(index, vector.len());
    }

    #[test]
    fn peek_first_test() {
        let vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.peek_first().is_none(), true);

        for element in &vector {
            list.add_last(*element);
        }

        let mut index = 0;
        loop {
            match (&list).peek_first() {
                None => break,
                Some(result) => {
                    assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
                    index += 1;
                }
            }
            list.pop_first();
        }

        assert_eq!(index, vector.len());
        assert_eq!((&list).count(), 0);
        assert_eq!(list.peek_first().is_none(), true);
    }

    #[test]
    fn pop_first_test() {
        let vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.pop_first().is_err(), true);
        for element in &vector {
            list.add_last(*element);
        }

        let mut index = 0;
        loop {
            match list.pop_first() {
                Err(_) => break,
                Ok(element) => assert_eq!(element, vector[index]),
            }
            index += 1;
        }
        assert_eq!(index, vector.len());
        assert_eq!((&list).count(), 0);
        assert_eq!(list.pop_first().is_err(), true);
    }

    #[test]
    fn peek_last_test() {
        let mut vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.peek_last().is_none(), true);

        for element in &vector {
            list.add_last(*element);
        }

        vector.reverse();

        let mut index = 0;
        loop {
            match (&list).peek_last() {
                None => break,
                Some(result) => {
                    assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
                    index += 1;
                }
            }
            list.pop_last();
        }

        assert_eq!(index, vector.len());
        assert_eq!((&list).count(), 0);
        assert_eq!(list.peek_last().is_none(), true);
    }

    #[test]
    fn pop_last_test() {
        let mut vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.pop_last().is_err(), true);

        for element in &vector {
            list.add_last(*element);
        }

        vector.reverse();

        let mut index = 0;
        loop {
            match list.pop_last() {
                Err(_) => break,
                Ok(result) => {
                    assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
                    index += 1;
                }
            }
        }

        assert_eq!(index, vector.len());
        assert_eq!((&list).count(), 0);
        assert_eq!(list.pop_last().is_err(), true);
    }

    #[test]
    fn add_first_test() {
        let mut vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        for element in &vector {
            list.add_first(*element);
        }
        assert_eq!((&list).count(), vector.len() as u32);

        vector.reverse();

        let mut index = 0;
        for element in list {
            assert_eq!(element, vector[index]);
            index += 1;
        }

        assert_eq!(index, vector.len());
    }

    #[test]
    fn extend_test() {
        let mut vector = vec![2, 5, 6, 3, 7, 10, 15];

        let mut list = LinkedList::<i32>::new();
        for element in &vector {
            list.add_last(*element);
        }

        let mut vector_clon = vector.clone();
        vector_clon.reverse();

        assert_eq!((&list).count(), vector.len() as u32);

        list.extend(vector_clon);

        for index in 0..vector.len() {
            assert_eq!(list.pop_first().unwrap_or(-1), vector[index]);
        }
        vector.reverse();
        for index in 0..vector.len() {
            assert_eq!(list.pop_first().unwrap_or(-1), vector[index]);
        }
        assert_eq!(list.pop_first().is_err(), true);
        assert_eq!((&list).count(), 0);
    }
}
