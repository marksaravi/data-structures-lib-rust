use data_structures_lib_rust::linked_list::{OneDirLinkedList, SimpleLinkedList};



fn main(){
    let mut list: SimpleLinkedList<i32> = SimpleLinkedList::new();

    assert!(list.is_empty());

    list.push(1);
    list.push(2);
    list.push(3);

    print_linked_list(&list);
}

fn print_linked_list(ll: &impl   OneDirLinkedList){
    println!("list length: {}", ll.length());
}