pub trait linked_list {
    type Item;
    fn new() -> Self;
    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<&Self::Item>;
    fn is_empty(&self) -> bool;
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
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
}
impl<T> linked_list for LinkedList<T> {
    type Item = T;

    fn new() -> Self {
        LinkedList::new()
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
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