mod single_linked_list;
use single_linked_list::SingleLinkedList;


fn main() {
    let my_list: SingleLinkedList<i32> = SingleLinkedList::new();
    assert_eq!(my_list.len(),0);

    let mut my_list: SingleLinkedList<i32> = SingleLinkedList::new();
    for i in 0..5 {
        my_list.push(i);
    }
    assert_eq!(my_list.len(),5);

    my_list.pop();
    assert_eq!(my_list.len(),4);

    my_list.append(5);
    my_list.append(6);
    assert_eq!(my_list.len(),6);
}
