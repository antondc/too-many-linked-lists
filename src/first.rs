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
    // { head: Link:More<Foo> }
    // [ Link::More<Foo> ], ( Foo, Link::Empty )

    let next = mem::replace(&mut self.head, Link::Empty);
    // 1. Replace self.head with Link::Empty:
    //    `[ Link::Empty ], ( Foo, Link::Empty )`.
    //
    // 2. Return the original head and store it in `next`:
    //    `next == [ Link::More<Foo> ]`.

    // ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )
    let new_node = Box::new(Node { elem, next });

    // [ Link::More<Bar> ], ( Bar, Link::More<Foo> ),   ( Foo, Link::Empty )
    self.head = Link::More(new_node);
  }

  // We empty head first returning it with `mem::replace`, and match against returned unallocated head
  pub fn pop(&mut self) -> Option<i32> {
    // [ Link::More<Bar> ], ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )

    let popped_node = mem::replace(&mut self.head, Link::Empty);
    // 1. Replace self.head with Link::Empty:
    //    `[ Link::Empty ], ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )`.
    //
    // 2. Return the original head and store it in `popped_node`:
    //    `popped_node == [ Link::More<Bar> ]`.

    match popped_node {
      Link::Empty => None, // List was empty, do nothing
      Link::More(node) => {
        self.head = node.next;
        // Node `Bar` popped:
        // Set head pointing to next node of second node —Foo—; second node dropped —Bar—:
        // [ Link::More<Foo> ], ( Foo, Link::Empty )
        Some(node.elem)
        // Return popped node
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
