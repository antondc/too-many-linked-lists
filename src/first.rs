use std::mem;

// -Tail of a list never allocates extra junk
// enum is in null-pointer-optimized form
// All elements are uniformly allocated
// Split List and Link to hide implementation details

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

    // Use of `mem::replace()` to:
    //
    // 1. Replace self.head with Link::Empty:
    //    `[ Link::Empty ], ( Foo, Link::Empty )`.
    //
    // 2. Return the head and store it in a `next` variable:
    //    `next == [ Link::More<Foo> ]`.
    //
    let next = mem::replace(&mut self.head, Link::Empty);

    // ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )
    let new_node = Box::new(Node { elem, next });

    // [ Link::More<Bar> ], ( Bar, Link::More<Foo> ),   ( Foo, Link::Empty )
    self.head = Link::More(new_node);
  }

  // We empty head first returning it with `mem::replace`, and match against returned unallocated head
  pub fn pop(&mut self) -> Option<i32> {
    // [ Link::More<Bar> ], ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )

    // Use of `mem::replace()` to:
    //
    // 1. Replace self.head with Link::Empty:
    //    `[ Link::Empty ], ( Bar, Link::More<Foo> ), ( Foo, Link::Empty )`.
    //
    // 2. Return the head and store it in a `next` variable:
    //    `popped_node == [ Link::More<Bar> ]`.
    //
    let popped_node = mem::replace(&mut self.head, Link::Empty);

    match popped_node {
      Link::Empty => None, // List was empty, do nothing
      Link::More(node) => {
        // Node `Bar` popped:
        // Set head pointing to next node of second node —Foo—; second node dropped —Bar—:
        // [ Link::More<Foo> ], ( Foo, Link::Empty )
        self.head = node.next;
        // Return popped node
        Some(node.elem)
      }
    }
  }
}
