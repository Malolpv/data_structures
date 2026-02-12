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

    linked_list.reverse();

    linked_list.print();

    linked_list.push_front(Box::new(Node::new(6)));
    linked_list.print();

    linked_list.pop_by_value(&2);

    linked_list.print();

    linked_list.push_back(Box::new(Node::new(100)));

    _ = linked_list.contains(100);

    linked_list.print();

    linked_list.clear();
    _ = linked_list.is_empty();
    linked_list.len();
    _ = linked_list.peek();
    _ = linked_list.peek_mut();
}
