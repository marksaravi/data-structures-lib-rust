use std::fmt;
use std::fmt::Display;

pub trait OneDirLinkedList {
    type Item;
    fn new() -> Self;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
    fn is_empty(&self) -> bool;
    fn length(&self) -> usize;
}
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}

impl<T> OneDirLinkedList for SimpleLinkedList<T> {
    type Item = T;

    fn new() -> Self {
        SimpleLinkedList::new()
    }

    fn push(&mut self, item: Self::Item) {
        let mut new_node = Box::new(Node::new(item));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    
    fn length(&self) -> usize {
        self.size()
    }
}

impl fmt::Display for SimpleLinkedList<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.length())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = SimpleLinkedList::new();
        assert!(list.is_empty());

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert!(!list.is_empty());

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert!(list.is_empty());
    }
}