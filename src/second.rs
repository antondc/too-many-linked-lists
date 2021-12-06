type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
  elem: T,
  next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
  head: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    // Replaced `mem::replace(&mut self.head, None)` with take()
    let original_head = self.head.take();

    let new_node = Box::new(Node {
      elem,
      next: original_head,
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    // Replaced `mem::replace(&mut self.head, None)` with take()
    // Use .map instead of match
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut current_link = self.head.take();

    while let Some(mut boxed_node) = current_link {
      current_link = boxed_node.next.take();
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