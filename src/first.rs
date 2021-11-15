// -Tail of a list never allocates extra junk
// enum is in null-pointer-optimized form
// All elements are uniformly allocated

struct Node {
  elem: i32,
  next: List,
}

pub enum List {
  Empty,
  More(Box<Node>),
}
