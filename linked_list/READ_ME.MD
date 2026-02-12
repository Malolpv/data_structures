# Linked List

## Singly Linked List
In the ``singly_linked_list`` sub module you will find a basic implementation of a Singly Linked List. This implementation leverages Rust's ownership model to provide a memory-safe, generic container.

### Core Structures
**Node<T>:** A container holding a value of type T and an optional pointer to the next node.

**SinglyLinkedList<T>:** The main structure managing the head of the list and tracking its length.

### Features and Methods
The current implementation provides several safe operations for managing data:
| Category   | Methods                                     | Description                                                |
|------------|---------------------------------------------|------------------------------------------------------------|
| Insertion  | ``push_front``, ``push_back``               | Add elements to the start or end of the list               |
| Removal    | ``pop_front``, ``pop_by_value``          | Remove the head or search for a specific value to extract. |
| Inspection | ``len`` ``is_empty``,``peek``, ``peek_mut`` | Access elements or metadata without consuming the list.    |
| Utilities  | ``clear``,``print``                         | Display the list or reset its contents.                    |

### Memory Management
The implementation includes a manual Drop trait. Without this, a long list would trigger recursive destruction of ``Box<Node<T>>``, potentially leading to a **stack overflow**. The manual implementation ensures nodes are dropped iteratively.
