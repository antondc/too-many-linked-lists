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

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.elem)
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

  #[test]
  fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| *value = 42);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
  }
}
