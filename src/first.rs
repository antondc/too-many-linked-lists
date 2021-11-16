use std::mem;

#[derive(Debug)]
enum Link {
  Empty,
  More(Box<Node>),
}

#[derive(Debug)]
struct Node {
  elem: i32,
  next: Link,
}

#[derive(Debug)]
pub struct List {
  head: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    // We start with a list that contains a value of `1` and we want to push a value of `2`:
    // `self == `List { head: More(Node { elem: 1, next: Empty }) }`
    // `elem == 2`

    let original_next = mem::replace(&mut self.head, Link::Empty);
    // 1. Replace self.head with Link::Empty:
    //    `self.head == Empty`.
    //
    // 2. Return the original head and store it in `original_next`:
    //    `original_next == More(Node { elem: 1, next: Empty })`.

    let new_node = Box::new(Node {
      elem,
      next: original_next,
    });
    // `new_node == Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }`

    self.head = Link::More(new_node);
    // `self.head == More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) })`
  }

  // We empty head first returning it with `mem::replace`, and match against returned unallocated head
  pub fn pop(&mut self) -> Option<i32> {
    // `self == List { head: More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }) }`

    let original_head = mem::replace(&mut self.head, Link::Empty);
    // 1. Replace self.head with Link::Empty:
    //    `self.head == Empty`.
    //
    // 2. Return the original head and store it in `original_head`:
    //    `original_head == More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) })`.

    // At this point `self.head == Empty`, we will set it in following match:
    match original_head {
      Link::Empty => None, // List was empty, do nothing
      Link::More(node) => {
        // `node` was the first node in the list
        // `node == Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }`
        self.head = node.next;
        // Set `node.next` as new head of the list
        // `self == List { head: More(Node { elem: 1, next: Empty }) }`
        // `self.head == More(Node { elem: 1, next: Empty })`
        // `node.next was moved and dropped`

        Some(node.elem)
        // Return value of `node.elem`, which was value of previous head
        // node.elem == 2
      }
    }
  }
}

impl Drop for List {
  fn drop(&mut self) {
    // `self == List { head: More(Node { elem: 3, next: More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }) }) }`

    let mut current_link = mem::replace(&mut self.head, Link::Empty);
    // 1. Replace self.head with Link::Empty:
    //    `self.head == Empty`.
    //
    // 2. Return the original head and store it in a `next` variable:
    //    `current_link == More(Node { elem: 3, next: More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }) })`.
    //

    // While extract the `Node` from `current_link`:
    while let Link::More(mut boxed_node) = current_link {
      // `boxed_node == Node { elem: 3, next: More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) }) }`

      current_link = mem::replace(&mut boxed_node.next, Link::Empty);
      // 1. Replace boxed_node.next with Link::Empty:
      //    `boxed_node == Node { elem: 3, next: Empty }`
      //
      // 2. Return original `boxed_node.next` and store it in `current_link` to iterate over it:
      //    `current_link == More(Node { elem: 2, next: More(Node { elem: 1, next: Empty }) })`.

      // Iterate…
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]

  fn basics() {
    let mut list = List::new();

    // Test that newly created list pop nothing
    assert_eq!(list.pop(), None);

    // Test that list push items, as well as pop them
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    // Test that list still behaves even after push/pop actions
    assert_eq!(list.pop(), None);
    list.push(123);
    list.push(43);
    assert_eq!(list.pop(), Some(43));
    assert_eq!(list.pop(), Some(123));
    assert_eq!(list.pop(), None);
  }
}
