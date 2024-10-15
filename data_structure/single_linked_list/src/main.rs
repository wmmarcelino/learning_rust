mod single_linked_list;
use single_linked_list::Node;


fn main() {

    let mut my_node = Node::new(1);
    println!("{:?}", my_node);
    my_node.get();
    //my_node.value = 3;
    //println!("{:?}", my_node);
//    let my_value = my_node;
   
//    println!("{:?}",my_node);

    // let mut my_list: SingleLinkedList<i32> = SingleLinkedList::new();
    // assert_eq!(my_list.len(),0);
    
    // for i in 0..5 {
    //     my_list.push(i);
    // }
    // assert_eq!(my_list.len(),5);

    // my_list.pop();
    // assert_eq!(my_list.len(),4);

    // my_list.append(5);
    // my_list.append(6);
    // assert_eq!(my_list.len(),6);
    // let mut x = my_list.head.take();
    // if let Some(next) = x.borrow_mut().take() {

    // }
    // println!("{}",x)
}
