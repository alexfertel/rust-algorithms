use std::{cell::RefCell, rc::Rc};

type Link<T> = Rc<RefCell<ListNode<T>>>;

fn create_link<T>(val: T) -> Link<T> {
    Rc::new(RefCell::new(ListNode::new(val)))
}

#[derive(PartialEq, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Link<T>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        Self { next: None, val }
    }
}

#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}

impl<T> Iterator for LinkedList<T> {
    type Item = Link<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = create_link(val);
        match self.head.take() {
            Some(link) => {
                new_head.borrow_mut().next = Some(link);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.tail = Some(new_head);
            }
        }
        self.length += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = create_link(val);
        match self.tail.take() {
            Some(link) => {
                link.borrow_mut().next = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }
        self.length += 1;
    }

    pub fn push_nth(&mut self, val: T, index: usize) {
        let new_link = create_link(val);
        match self.peek_nth(index) {
            Some(nth) => {
                nth.borrow_mut().next = Some(Rc::clone(&new_link));

                match nth.borrow_mut().next.take() {
                    Some(next) => {
                        new_link.borrow_mut().next = Some(next);
                    }
                    None => {
                        self.tail = Some(Rc::clone(&new_link));
                    }
                }
            }
            None => {
                self.head = Some(Rc::clone(&new_link));
                self.tail = Some(new_link);
            }
        }
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<Link<T>> {
        match self.head.take() {
            Some(head) => {
                self.head = head.borrow_mut().next.take();
                self.length -= 1;
                Some(head)
            }
            None => None,
        }
    }

    pub fn pop_back(&mut self) -> Option<Link<T>> {
        match self.tail.take() {
            Some(tail) => {
                let mut new_tail = self.head.take().unwrap();
                self.head = Some(Rc::clone(&new_tail));

                for _ in 0..self.length {
                    let temp_ptr = match &new_tail.borrow().next {
                        Some(val) => match &val.borrow().next {
                            Some(_) => Some(Rc::clone(&val)),
                            None => {
                                break;
                            }
                        },
                        None => {
                            break;
                        }
                    };
                    new_tail = temp_ptr.unwrap();
                }

                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
                self.length -= 1;
                Some(tail)
            }
            None => None,
        }
    }

    pub fn pop_nth(&mut self, index: usize) -> Option<Link<T>> {
        if index >= self.len() {
            return None;
        } else if index == 0 {
            return self.pop_front();
        } else if index == self.len() - 1 {
            return self.pop_back();
        }

        let nth = self.peek_nth(index);

        self.peek_nth(index - 1).unwrap().borrow_mut().next = self.peek_nth(index + 1);
        self.length -= 1;
        nth
    }

    pub fn peek_front(&self) -> Option<Link<T>> {
        match &self.head {
            Some(head) => Some(Rc::clone(head)),
            None => None,
        }
    }

    pub fn peek_back(&self) -> Option<Link<T>> {
        match &self.tail {
            Some(tail) => Some(Rc::clone(tail)),
            None => None,
        }
    }

    pub fn peek_nth(&mut self, index: usize) -> Option<Link<T>> {
        if index >= self.len() || index == 0 {
            return self.peek_front();
        } else if index == self.len() - 1 {
            return self.peek_back();
        }

        let mut pointer = self.head.take().unwrap();
        self.head = Some(Rc::clone(&pointer));

        for _ in 0..index {
            let next = pointer.borrow_mut().next.take();
            pointer = next.unwrap();
        }

        Some(pointer)
    }
}

#[cfg(test)]
mod test {
    use super::{create_link, LinkedList};

    fn create_list<T>(arr: &[T]) -> LinkedList<T>
    where
        T: Clone,
    {
        let mut list = LinkedList::new();
        for num in arr.iter() {
            list.push_back((*num).clone());
        }
        list
    }

    #[test]
    fn push_front_test() {
        let mut test_list = create_list(&[1, 2]);
        let mut empty_list: LinkedList<i32> = LinkedList::new();
        test_list.push_front(0);
        empty_list.push_front(0);
        assert_eq!(create_list(&[0, 1, 2]), test_list);
        assert_eq!(create_list(&[0]), empty_list);
    }

    #[test]
    fn push_back_test() {
        let mut test_list = create_list(&[0, 1]);
        let mut empty_list: LinkedList<i32> = LinkedList::new();
        test_list.push_back(2);
        empty_list.push_back(0);
        assert_eq!(create_list(&[0, 1, 2]), test_list);
        assert_eq!(create_list(&[0]), empty_list);
    }

    #[test]
    fn pop_front_test() {
        let mut test_list = create_list(&[0, 1, 2]);
        assert_eq!(Some(create_link(0)), test_list.pop_front());
        assert_eq!(create_list(&[1, 2]), test_list);
        assert_eq!(None, LinkedList::<i32>::new().pop_front());
    }

    #[test]
    fn pop_back_test() {
        let mut test_list = create_list(&[0, 1, 2]);
        assert_eq!(Some(create_link(2)), test_list.pop_back());
        assert_eq!(create_list(&[0, 1]), test_list);
        assert_eq!(None, LinkedList::<i32>::new().pop_back());
    }

    #[test]
    fn pop_nth_test() {
        let mut test_list = create_list(&[0, 1, 2]);
        assert_eq!(create_list(&[1, 2]).peek_front(), test_list.pop_nth(1));
        assert_eq!(None, LinkedList::<i32>::new().pop_nth(1));
    }

    #[test]
    fn peek_front_test() {
        let test_list = create_list(&[0, 1, 2]);
        let link = create_link(0);

        link.borrow_mut().next = Some(create_link(1));
        link.borrow_mut().next.as_ref().unwrap().borrow_mut().next = Some(create_link(2));

        assert_eq!(Some(link), test_list.peek_front());
        assert_eq!(None, LinkedList::<i32>::new().peek_front());
    }

    #[test]
    fn peek_back_test() {
        let test_list = create_list(&[0, 1, 2]);
        assert_eq!(Some(create_link(2)), test_list.peek_back());
        assert_eq!(None, LinkedList::<i32>::new().peek_back());
    }

    #[test]
    fn peek_nth_test() {
        let mut test_list = create_list(&[0, 1, 2]);
        assert_eq!(create_list(&[1, 2]).peek_front(), test_list.peek_nth(1));
        assert_eq!(None, LinkedList::<i32>::new().peek_nth(1));
    }

    #[test]
    fn iter_test() {
        let arr = &[0, 1, 2];
        let test = create_list(arr);

        for (i, node) in test.into_iter().enumerate() {
            assert_eq!(node.borrow().val, arr[i])
        }
    }
}
