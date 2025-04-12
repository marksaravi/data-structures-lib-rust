use data_structures_lib_rust::linked_list::{OneWayLinkedList, SimpleLinkedList};





fn main(){
    let mut list: SimpleLinkedList<i32> = OneWayLinkedList::new();
    let mut list2: SimpleLinkedList<i32> = SimpleLinkedList::new();
    assert!(list.is_empty());

    list.push(1);
    list.push(2);
    list.push(3);
    println!("{}", list2.size());
}