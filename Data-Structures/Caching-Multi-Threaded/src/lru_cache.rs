use std::{collections::HashMap, hash::Hash};

use crate::doubly_linked_list::{DLL, Link};

/*
    using map for faster value access, O(1) lookup
        - Key will be actual item (can be string, int whatever), that we are storing to cache.
        - value will be the pointer to the Node, where each node represent the

    using DoublyLinkedList to track frequency
        - frequently used item in the front
        - least frequently used item in the back
*/
pub struct LRUCache<K, T> {
    pub capacity: usize,
    pub map: HashMap<K, Link<K, T>>,
    pub cache: DLL<K, T>,
}

impl<K, T> LRUCache<K, T>
where
    K: Copy + Eq + Hash,
    T: Copy,
{
    pub fn new(capacity: usize) -> Self {
        let dll: DLL<K, T> = DLL::new();
        Self {
            capacity,
            map: HashMap::new(),
            cache: dll,
        }
    }

    pub fn put(&mut self, key: K, value: T) {
        if let Some(Some(node)) = self.map.get(&key).clone() {
            node.borrow_mut().value = value;
            self.cache.move_to_front(&node)
        } else {
            if self.map.len() >= self.capacity {
                if let Some(key) = self.cache.remove_tail() {
                    self.map.remove(&key);
                }
            }
            self.cache.insert_to_front(key, value);
            self.map.insert(key, self.cache.head.clone());
        }
    }

    pub fn get(&mut self, key: K) -> Option<T> {
        let node = self.map.get(&key)?.clone()?;
        let value = node.borrow().value;
        self.cache.move_to_front(&node);
        return Some(value);
    }

    pub fn remove(&mut self, key: K) {
        if let Some(Some(node)) = self.map.get(&key) {
            self.cache.remove(node);
            self.map.remove(&key);
        }
    }
}
