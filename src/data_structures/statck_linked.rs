use std::path::Iter;
use std::vec::IntoIter;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    element: T,
    next: Link<T>
}

pub struct Stack<T> {
    head: Link<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take()
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        match self.head.take() {
            None => Err("Stack is empty"),
            Some(node) => {
                self.head = node.next;
                Ok(node.element)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            None => None,
            Some(node) => Some(&node.element)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.head.as_mut() {
            None => None,
            Some(node) => Some(&mut node.element)
        }
    }

    pub fn into_iter_for_stack(&self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    // This is declaring that every implementation of iterator has an associated type called Item
    type Item = T;
    // the reason iterator yield Option<self::Item> is because the interface coalesces the `has_next` and `get_next` concepts
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop().ok()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // as_deref: Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take()
        }
    }
}



pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // we add take() here due to &mut self isn't Copy(& and Option<&> is Copy)
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


impl<T> Default for Stack<T> {
    fn default() -> Self {
       Self::new()
    }
}


#[cfg(test)]
mod test_stack {

    use super::*;

    #[test]
    fn basics() {
        let mut list = Stack::new();
        assert_eq!(list.pop(), Err("Stack is empty"));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Ok(3));
        assert_eq!(list.pop(), Ok(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.is_empty(), false);

        assert_eq!(list.pop(), Ok(5));
        assert_eq!(list.pop(), Ok(4));

        assert_eq!(list.pop(), Ok(1));
        assert_eq!(list.pop(), Err("Stack is empty"));

        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        match list.peek_mut() {
            None => None,
            Some(value) => Some(*value = 42),
        };

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Ok(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter_for_stack();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}