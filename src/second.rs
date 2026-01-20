use std::mem;
use Drop;

pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self { List { head: None } }

    pub fn push(&mut self, e: T) {
        let new_node = Box::new(Node { elem: e, next: self.head.take(), });
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node|  {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|  {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut boxed_node) = link {
            link = mem::replace(&mut boxed_node.next, None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(22);
        list.push(12);
        list.push(34);

        assert_eq!(list.pop(), Some(34));
        assert_eq!(list.pop(), Some(12));

        list.push(55);
        list.push(99);

        assert_eq!(list.pop(), Some(99));
        assert_eq!(list.pop(), Some(55));
        assert_eq!(list.pop(), Some(22));
    }

    #[test]
    fn peek() {
        let mut list : List<i32> = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(22);
        assert_eq!(list.peek(), Some(&22));
        assert_eq!(list.peek_mut(), Some(&mut 22));
        list.peek_mut().map(|value| {
            *value = 45
        });
    }

    #[test]
    fn into_iter() {
        let mut list: List<i32> = List::new();

        list.push(22); list.push(12); list.push(34);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(34));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(22));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list: List<i32> = List::new();
        list.push(11); list.push(22); list.push(33); list.push(44);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&44));
        assert_eq!(iter.next(), Some(&33));
        assert_eq!(iter.next(), Some(&22));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list: List<i32> = List::new();
        list.push(11); list.push(22); list.push(33); list.push(44);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 44));
        assert_eq!(iter.next(), Some(&mut 33));
        assert_eq!(iter.next(), Some(&mut 22));
        assert_eq!(iter.next(), Some(&mut 11));
        assert_eq!(iter.next(), None);
    }
}