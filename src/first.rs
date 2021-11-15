// -Tail of a list never allocates extra junk
// enum is in null-pointer-optimized form
// All elements are uniformly allocated
// Split List and Link to hide implementation details

pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}
