//! # Linked List Module
//!
//! This module defines a thread-safe singly linked list using `Arc` and `Mutex`.
//! Each node stores an `Arc<Mutex<T>>` value, and the list supports appending,
//! popping, and iteration over values.

use std::sync::{Arc, Mutex};

use sdl2::keyboard::{Keycode, Mod};

/// A node in the singly linked list.
///
/// Each node stores an `Arc<Mutex<T>>` value and a link to the next node.
pub struct Node<T> {
    /// The value stored in this node.
    pub value: Option<Arc<Mutex<T>>>,
    /// The next node in the list, if any.
    pub next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    /// Creates a new node with the given value.
    ///
    /// The value is wrapped in an `Arc<Mutex<T>>` for thread-safe sharing.
    pub fn new(value: T) -> Self {
        Self {
            value: Some(Arc::new(Mutex::new(value))),
            next: None,
        }
    }
}

/// An iterator over references to the values (`Arc<Mutex<T>>`) stored in a `List<T>`.
pub struct ListIter<'a, T> {
    current: Option<Arc<Mutex<Node<T>>>>,
    marker: std::marker::PhantomData<&'a T>,
}

/// A thread-safe singly linked list.
///
/// The list supports appending new values, popping values from the head,
/// checking if it is empty, and iteration by reference.
#[derive(Default)]
pub struct List<T> {
    /// The first node in the list, if any.
    pub head: Option<Arc<Mutex<Node<T>>>>,
    /// The last node in the list, if any.
    pub tail: Option<Arc<Mutex<Node<T>>>>,
    /// The number of elements in the list.
    pub length: Mutex<u32>,
    /// The current node used for internal iteration (`jump` method).
    pub current: Option<Arc<Mutex<Node<T>>>>,
}

impl<T> List<T> {
    /// Creates a new empty list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: Mutex::new(0),
            current: None,
        }
    }

    /// Returns an iterator over the list by reference.
    ///
    /// The iterator yields `Arc<Mutex<T>>` values.
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_ref().map(Arc::clone),
            marker: std::marker::PhantomData,
        }
    }

    /// Moves the internal cursor forward and returns the next node (internal use).
    ///
    /// If the cursor is not set, it starts from the head.
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

    /// Appends a new value to the end of the list.
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

    /// Removes and returns the value at the head of the list.
    ///
    /// Returns an error if the list is empty.
    pub fn pop(&mut self) -> Result<Arc<Mutex<T>>, String> {
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

    /// Returns `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Increments the internal length counter.
    fn increment(&mut self) {
        let mut length = self.length.lock().unwrap();
        *length += 1;
    }

    /// Decrements the internal length counter.
    fn decrement(&mut self) {
        let mut length = self.length.lock().unwrap();
        if *length > 0 {
            *length -= 1;
        }
    }
}

impl<'a, T> Iterator for ListIter<'a, T> {
    /// The type of value returned when iterating: `Arc<Mutex<T>>`.
    type Item = Arc<Mutex<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.current.as_ref().map(Arc::clone) {
            let node = curr.lock().unwrap();

            // clone the Arc<Mutex<T>> stored in value
            let value = node.value.as_ref().map(Arc::clone);

            // move to next node
            self.current = node.next.clone();

            value
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    /// The type of value returned when iterating: `Arc<Mutex<T>>`.
    type Item = Arc<Mutex<T>>;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub mod state_machines {
    use super::KeyAction;
    use crate::engine::traits::{PhysicsObjectTrait, StaticObjectTrait};
    use once_cell::sync::Lazy;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, RwLock},
    };

    /// Central state machine for handling input actions.
    #[derive(Default)]
    pub struct InputAction {
        stack: Arc<Mutex<Option<KeyAction>>>,
    }

    impl InputAction {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn push(&self, action: KeyAction) {
            self.stack.lock().unwrap().replace(action);
        }

        pub fn pop(&self) -> Option<KeyAction> {
            self.stack.lock().unwrap().take()
        }
    }

    /// Thread-safe, Lazily-initialized input action state shared across the program.
    pub static INPUT_ACTION: Lazy<Arc<RwLock<InputAction>>> =
        Lazy::new(|| Arc::new(RwLock::new(InputAction::new())));

    /// Public API to get input action from stack.
    pub fn get_current_input_action() -> Option<KeyAction> {
        INPUT_ACTION
            .read()
            .map_err(|e| format!("RwLock poisoned: {}", e))
            .unwrap()
            .pop()
    }

    /// Public API to push input action to stack.
    pub fn push_input_action(action: KeyAction) {
        INPUT_ACTION
            .write()
            .map_err(|e| format!("RwLock poisoned: {}", e))
            .unwrap()
            .push(action);
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
        assert_eq!(*list.pop().unwrap().lock().unwrap(), 1);
        assert_eq!(*list.pop().unwrap().lock().unwrap(), 2);
        assert_eq!(*list.pop().unwrap().lock().unwrap(), 3);

        // Now list should be empty again
        assert!(list.is_empty());

        // Length should be 0
        let length = *list.length.lock().unwrap();
        assert_eq!(length, 0);

        // Popping from empty list should error
        assert!(list.pop().is_err());
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

        for value in &list {
            counter += 1;
            let val = value.lock().unwrap();
            assert_eq!(*val, counter);
        }

        let length2 = *list.length.lock().unwrap();

        assert!(length2 == 3);
    }
}

#[derive(Debug, Clone)]
pub struct KeyAction {
    pub keycode: Keycode,
    pub window_id: u32,
    pub key_mod: Mod,
    pub repeat: bool,
    pub timestamp: u32,
}

impl KeyAction {
    pub fn new(
        window_id: u32,
        keycode: Keycode,
        key_mod: Mod,
        repeat: bool,
        timestamp: u32,
    ) -> Self {
        Self {
            keycode,
            window_id,
            key_mod,
            repeat,
            timestamp,
        }
    }
}
