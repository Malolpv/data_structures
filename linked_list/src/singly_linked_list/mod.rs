use std::fmt::Display;

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Create a new node
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    /// Create a new empty list
    pub fn new() -> Self {
        Self {
            head: None,
            len: usize::MIN,
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Insert a new node at the begining of the list
    pub fn push_front(&mut self, mut node: Box<Node<T>>) {
        if let Some(tmp) = self.head.take() {
            node.next = Some(tmp);
            self.head = Some(node);
        } else {
            self.head = Some(node)
        }
        self.len += 1
    }

    /// Remove the last inserted node and return its value or none
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|some| {
            self.head = some.next;
            self.len -= 1;
            some.value
        })
    }

    /// Return true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear the list of any nodes
    pub fn clear(&mut self) {
        todo!()
    }

    /// Push the given node to the end of the list
    pub fn push_back(&mut self, node: Box<Node<T>>) {
        let mut current = self.head.as_mut();

        if current.is_none() {
            self.head = Some(node);
        } else {
            while let Some(n) = current {
                if n.next.is_none() {
                    n.next = Some(node);
                    break;
                }

                current = n.next.as_mut();
            }
        }

        // Increment len counter
        self.len += 1;
    }

    /// Look at the first node value without modifying it
    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = self.head.as_ref() {
            Some(&node.value)
        } else {
            None
        }
    }

    /// Look at the first node value and return a mutable reference
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if let Some(node) = self.head.as_mut() {
            Some(&mut node.value)
        } else {
            None
        }
    }

    /// Reverse the list
    pub fn reverse(&mut self) {
        todo!()
    }
}

impl<T: PartialEq> SinglyLinkedList<T> {
    /// Traverse the list remove and return the first node with a matching value
    pub fn pop_by_value(&mut self, value: &T) -> Option<T> {
        let mut current = &mut self.head;

        // traverse the list
        while let Some(node) = current {
            // Found the value
            if node.value == *value {
                // Swap position
                let removed_node = current.take()?;
                *current = removed_node.next;

                // Update list counter and return
                self.len -= 1;
                return Some(removed_node.value);
            }
            current = &mut current.as_mut()?.next
        }

        None
    }

    /// Traverse all list elements to find given value
    pub fn contains(&self) -> bool {
        todo!()
    }

    /// Sort the list
    pub fn sort(&mut self) {
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
    pub fn print(&self) {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn len_empty_list() {
        // Arrange
        let list = SinglyLinkedList::<u16>::new();

        // Act
        let result = list.len();

        // Assert
        assert!(list.is_empty());
        assert_eq!(result, 0);
    }

    #[test]
    fn len_non_empty_list() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let n3 = Node::new(3);

        list.push_front(Box::new(n3));
        list.push_front(Box::new(n2));
        list.push_front(Box::new(n1));

        let _ = list.pop_front();

        // Act
        let result = list.len();

        // Assert
        assert!(!list.is_empty());
        assert_eq!(result, 2);
    }

    #[test]
    fn peek_empty_list() {
        // Arrange
        let list = SinglyLinkedList::<u16>::new();

        // Act
        let result = list.peek();

        // Assert
        assert_eq!(list.len(), 0);
        assert_eq!(result, None);
    }

    #[test]
    fn peek_non_empty_list() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let n3 = Node::new(3);

        list.push_front(Box::new(n3));
        list.push_front(Box::new(n2));
        list.push_front(Box::new(n1));

        // Act
        let result = list.peek();

        // Assert
        assert_eq!(list.len(), 3);
        assert_eq!(result, Some(&1));
    }

    #[test]
    fn peek_mut() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        let n1 = Node::new(1);
        let n2 = Node::new(2);

        list.push_front(Box::new(n2));
        list.push_front(Box::new(n1));

        // Act
        let result = list.peek_mut().unwrap();
        *result = 10;

        // Assert
        assert_eq!(list.peek(), Some(&10));
    }

    #[test]
    fn peek_mut_empty_list() {
        // Arrange
        let mut list = SinglyLinkedList::<i32>::new();

        // Act
        let result = list.peek_mut();

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn push_front() {
        // Arrange
        let node: Node<u16> = Node::new(10);
        let node2: Node<u16> = Node::new(15);
        let mut list = SinglyLinkedList::new();

        // Act
        list.push_front(Box::new(node));
        list.push_front(Box::new(node2));

        // Assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.peek(), Some(&15));
    }

    #[test]
    fn test_pop_front() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        list.push_front(Box::new(Node::new(1)));
        list.push_front(Box::new(Node::new(2)));

        // Act & Assert
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_pop_missing_value() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        list.push_front(Box::new(Node::new(1)));

        // Act
        let result = list.pop_by_value(&42);

        // Assert
        assert!(result.is_none());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_pop_existing_value() {
        // Arrange
        let mut list = SinglyLinkedList::new();
        list.push_front(Box::new(Node::new(1)));
        list.push_front(Box::new(Node::new(2)));
        list.push_front(Box::new(Node::new(3)));

        // Act
        let result = list.pop_by_value(&1);

        // Assert
        assert_eq!(list.len(), 2);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_empty_list_operations() {
        // Arrange
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();

        // Act & Assert
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_push_back() {
        // Arrange
        let mut list = SinglyLinkedList::new();

        // Act
        list.push_back(Box::new(Node::new(5)));
        list.push_back(Box::new(Node::new(15)));

        // Assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(15));
    }
}
