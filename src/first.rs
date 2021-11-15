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
    // Use of mem::replace to replace self.head temporarily with Link::Empty before replacing it again with the new head of the list.
    // 1. [ Link::More<Foo> ], ( "Foo", Link::Empty )
    // 2. [ Link::More<Bar> ], ( "Bar", Link::More<Foo> ), ( "Foo", Link::Empty )
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
  }
  // We empty head first returning it with `mem::replace`, and match against returned unallocated head
  // 1. [ Link::More<Bar> ], ( "Bar", Link::More<Foo> ), ( "Foo", Link::Empty )
  // 2. [ Link::More<Foo> ], ( "Foo", Link::Empty )
  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}
