use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Create a new node
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    /// Create a new empty list
    fn new() -> Self {
        Self {
            head: None,
            len: usize::MIN,
        }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.len
    }

    /// Insert a new node at the begining of the list
    fn push_front(&mut self, mut node: Box<Node<T>>) {
        if let Some(tmp) = self.head.take() {
            node.next = Some(tmp);
            self.head = Some(node);
        } else {
            self.head = Some(node)
        }
        self.len += 1
    }

    /// Remove the last inserted node and return its value or none
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|some| {
            self.head = some.next;
            self.len -= 1;
            some.value
        })
    }

    /// Return true if the list is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear the list of any nodes
    fn clear(&mut self) {
        todo!()
    }

    /// Push the given node to the end of the list
    fn push_back(&self, node: Box<Node<T>>) {
        todo!()
    }

    /// Look at the first node value without modifying it
    fn peek(&self) -> Option<&T> {
        if let Some(node) = self.head.as_ref() {
            Some(&node.value)
        } else {
            None
        }
    }

    /// Look at the first node value and return a mutable reference
    fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(node) = self.head.as_mut() {
            Some(&mut node.value)
        } else {
            None
        }
    }

    /// Reverse the list
    fn reverse(&mut self) {
        todo!()
    }
}

impl<T: PartialEq> SinglyLinkedList<T> {
    /// Traverse the list remove and return the first node with a matching value
    fn remove_by_value(&mut self, value: &T) -> Option<Box<Node<T>>> {
        let mut current = &mut self.head;

        while current.is_some() {
            if current.as_ref().unwrap().value == *value {
                let mut removed_node = current.take();

                if let Some(ref mut node) = removed_node {
                    *current = node.next.take();
                }

                self.len -= 1;
                return removed_node;
            }

            current = &mut current.as_mut().unwrap().next;
        }

        None
    }

    /// Traverse all list elements to find given value
    fn contains(&self) -> bool {
        todo!()
    }

    /// Sort the list
    fn sort(&mut self) {
        todo!()
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    /// Without this implementation, Box will call its drop recursively resulting in a stack overflow.
    ///
    /// This implementation ensure that this situation cannot happen by manually dropping each node of the linked list
    fn drop(&mut self) {
        let mut current = self.head.take();

        while let Some(mut node) = current {
            current = node.next.take()
        }
    }
}

/// TODO implement this :
/// IntoIter : Consomme la liste (prend l'ownership). Rend des T.
/// Iter : Emprunte la liste. Rend des &T.
/// IterMut : Emprunte la liste de façon mutable. Rend des &mut T.

impl<T: Display> SinglyLinkedList<T> {
    fn print(&self) {
        let mut string = String::new();
        let mut next = self.head.as_ref();

        while let Some(node) = next {
            // Print current value
            string.push_str(format!("{}\t->", node.value).as_str());

            // update next pointer
            next = node.next.as_ref()
        }
        println!("List contains: {} element(s)", self.len);
        println!("{} None", string)
    }
}

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

    linked_list.remove_by_value(&2);

    linked_list.print();
}
