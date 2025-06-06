use std::sync::{Arc, Mutex};

pub struct Node<T> {
    pub value: Option<T>,
    pub next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            next: None,
        }
    }
}

pub struct ListIter<'a, T> {
    current: Option<Arc<Mutex<Node<T>>>>,
    marker: std::marker::PhantomData<&'a T>,
}

#[derive(Default)]
pub struct List<T> {
    pub head: Option<Arc<Mutex<Node<T>>>>,
    pub tail: Option<Arc<Mutex<Node<T>>>>,
    pub length: Mutex<u32>,
    pub current: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: Mutex::new(0),
            current: None,
        }
    }
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_ref().map(Arc::clone),
            marker: std::marker::PhantomData,
        }
    }

    pub fn jump(&mut self) -> Option<Arc<Mutex<Node<T>>>> {
        if self.current.is_none() {
            if let Some(head_arc) = self.head.as_ref().map(Arc::clone) {
                self.current = Some(head_arc);
                return self.current.clone();
            }
        }

        if let Some(current_arc) = self.current.as_ref().map(Arc::clone) {
            let current = current_arc.lock().unwrap();
            self.current = current.next.clone();
            return self.current.clone();
        }
        None
    }

    pub fn append(&mut self, value: T) {
        let new_node = Arc::new(Mutex::new(Node::new(value)));

        // TODO: add error handling later
        if self.head.is_some() {
            if let Some(tail_arc) = self.tail.as_ref().map(Arc::clone) {
                let mut tail_node = tail_arc.lock().unwrap();
                tail_node.next = Some(Arc::clone(&new_node));
                self.increment();
                self.tail = Some(new_node);
            }
        } else {
            self.increment();
            self.head = Some(Arc::clone(&new_node));
            self.tail = Some(new_node)
        }
    }

    pub fn pop(&mut self) -> Result<T, String> {
        if let Some(head_arc) = self.head.as_ref().map(Arc::clone) {
            let mut node = head_arc.lock().unwrap();
            self.decrement();
            self.head = node.next.clone();

            if self.head.is_none() {
                self.tail = None;
            }

            let value = node.value.take().unwrap();

            return Ok(value);
        }

        Err("failed".to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn increment(&mut self) {
        let mut length = self.length.lock().unwrap();
        *length += 1;
    }

    fn decrement(&mut self) {
        let mut length = self.length.lock().unwrap();
        if *length > 0 {
            *length -= 1;
        }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = Arc<Mutex<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = &self.current {
            let next_node = curr.lock().unwrap().next.clone();
            let result = Some(Arc::clone(curr));
            self.current = next_node;
            result
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = Arc<Mutex<Node<T>>>;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_pop() {
        let mut list = List::new();

        // Initially empty
        assert!(list.is_empty());

        // Append values
        list.append(1);
        list.append(2);
        list.append(3);

        // List should not be empty
        assert!(!list.is_empty());

        // Length should be 3
        let length = *list.length.lock().unwrap();
        assert_eq!(length, 3);

        // Pop values (should be FIFO for your current design: singly linked list popping head)
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 3);

        // Now list should be empty again
        assert!(list.is_empty());

        // Length should be 0
        let length = *list.length.lock().unwrap();
        assert_eq!(length, 0);

        // Popping from empty list should error
        assert!(list.pop().is_err());
    }

    #[test]
    fn test_jump() {
        let mut list = List::new();

        assert!(list.is_empty());

        list.append(1);
        list.append(2);
        list.append(3);

        let length = *list.length.lock().unwrap();
        assert!(length == 3);

        assert!(list.jump().unwrap().lock().unwrap().value.unwrap() == 1);

        let length2 = *list.length.lock().unwrap();

        assert!(length2 == 3)
    }

    #[test]
    fn test_iterator() {
        let mut list = List::new();

        assert!(list.is_empty());

        list.append(1);
        list.append(2);
        list.append(3);

        let length = *list.length.lock().unwrap();
        assert!(length == 3);

        let mut counter = 0;

        for node in &list {
            counter += 1;
            assert!(node.lock().unwrap().value.unwrap() == counter);
        }

        let length2 = *list.length.lock().unwrap();

        assert!(length2 == 3)
    }
}
