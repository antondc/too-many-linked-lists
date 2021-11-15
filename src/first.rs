use std::mem;

// -Tail of a list never allocates extra junk
// enum is in null-pointer-optimized form
// All elements are uniformly allocated
// Split List and Link to hide implementation details

#[derive(Debug)]
pub struct List {
  head: Link,
}

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

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    // Use of mem::replace to replace self.head temporarily with Link::Empty before replacing it with the new head of the list.
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
  }
}
