use std::mem;
use Drop;

pub mod first;

pub struct List {
    head: Link,
}
impl Drop for List {
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = link {
            link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

struct Node {
    elem: i32,
    next: Link
}
enum Link {
    Empty,
    More(Box<Node>)
}

impl List {
    pub fn new() -> Self { List { head: Link::Empty } }

    pub fn push(&mut self, e: i32) {
        let new_node = Box::new(Node { elem: e, next: mem::replace(&mut self.head, Link::Empty) , });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::List;

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
}
