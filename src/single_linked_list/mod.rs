use std::rc::Rc;
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<NodeRef<T>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value:T) -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}


#[derive(Debug)]
pub struct SingleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: u64,
}

impl<T> SingleLinkedList<T> {

    pub fn new() -> SingleLinkedList<T> {
        SingleLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value:T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(last_node) => last_node.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }
        self.length += 1;
        self.tail = Some(new.clone());
    }

    pub fn push(&mut self, value: T) {
        let new = Node::new(value);
        match self.head.take() {
            Some(first_node) => new.borrow_mut().next = Some(first_node.clone()),
            None => self.tail = Some(new.clone())
        }
        self.length += 1;
        self.head = Some(new.clone());
    }
    
    pub fn pop(&mut self) -> Link<T> {
        match self.head.take() {
            Some(first_node) => {
                if self.length == 1 {
                    self.tail = None;
                }
                else {
                    self.head = first_node.borrow_mut().next.clone();
                    first_node.borrow_mut().next = None;
                }
                self.length -= 1;
                Some(first_node)

            }
            None => None
        }
    }

    pub fn len(&self) -> u64{
        self.length
    }

}