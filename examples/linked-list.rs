use data_structures_lib_rust::linked_list::{OneDirLinkedList, SimpleLinkedList};



fn main(){
    let mut list: SimpleLinkedList<i32> = OneDirLinkedList::new();

    assert!(list.is_empty());

    list.push(1);
    list.push(2);
    list.push(3);
    println!("list.size(): {}", list.size());

}