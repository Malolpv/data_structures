mod singly_linked_list;

use singly_linked_list::{Node, SinglyLinkedList};

fn main() {
    let mut linked_list = SinglyLinkedList::<u16>::new();

    linked_list.print();

    linked_list.push_front(Box::new(Node::new(1)));
    linked_list.push_front(Box::new(Node::new(2)));
    linked_list.push_front(Box::new(Node::new(6)));
    linked_list.push_front(Box::new(Node::new(3)));
    linked_list.push_front(Box::new(Node::new(10)));

    linked_list.print();

    _ = linked_list.pop_front();

    linked_list.print();

    linked_list.push_front(Box::new(Node::new(6)));
    linked_list.print();

    linked_list.pop_by_value(&2);

    linked_list.print();

    linked_list.push_back(Box::new(Node::new(100)));

    linked_list.print();
}
