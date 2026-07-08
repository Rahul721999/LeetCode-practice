use std::{cell::RefCell, rc::Rc};

pub type Link<K, T> = Option<Rc<RefCell<Node<K, T>>>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<K, T> {
    pub key: K,
    pub value: T,
    pub next: Link<K, T>,
    pub prev: Link<K, T>,
}
impl<K, T> Node<K, T> {
    pub fn new(key: K, value: T) -> Rc<RefCell<Node<K, T>>> {
        Rc::new(RefCell::new(Self {
            key,
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DLL<K, T> {
    pub head: Link<K, T>, // Most recent used
    pub tail: Link<K, T>, // Least recent used
}

impl<K, T> DLL<K, T> 
    where
        K: Copy + Clone,
        T: Copy + Clone,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn insert_to_front(&mut self, key: K, value: T) {
        let temp_head = self.head.take();
        let new_node = Node::new(key, value);

        if let Some(node) = temp_head {
            node.borrow_mut().prev = Some(Rc::clone(&new_node));
        } else {
            self.tail = Some(Rc::clone(&new_node))
        }

        self.head = Some(new_node);
    }

    pub fn remove(&mut self, node_to_remove: &Rc<RefCell<Node<K,T>>>) {

        let mut node = node_to_remove.borrow_mut(); // safer to use one mut_borrow and then drop
        let prev = node.prev.take();
        let next = node.next.take();
        drop(node);

        if let Some(node) = prev.as_ref() {
            node.borrow_mut().next = next.clone();
        }else{ // removing the 1st node
            self.head = next.clone();
        }

        if let Some(node) = next.as_ref(){
            node.borrow_mut().prev = prev;
        }else{ // removing the last node
            self.tail = prev.clone();
        }
    }
    pub fn move_to_front(&mut self, node: &Rc<RefCell<Node<K,T>>>) {
        let node_to_reposition = Rc::clone(node);
        self.remove(node);
        node_to_reposition.borrow_mut().prev = None; // important to handle

        let temp_head = self.head.take();
        if let Some(old_head) = temp_head{
            old_head.borrow_mut().prev = Some(Rc::clone(&node_to_reposition));
            node_to_reposition.borrow_mut().next = Some(Rc::clone(&old_head));
        }else{
            self.tail = Some(Rc::clone(&node_to_reposition));
        }
        self.head = Some(node_to_reposition);
    }
    
    pub fn remove_tail(&mut self) -> Option<K> {
        // if tail was pointing to some node
        if let Some(node) = self.tail.take(){
            let key = node.borrow().key;
            // get the (n-1)th node
            let mut prev = node.borrow_mut().prev.take();
            
            // if (n-1) node is some node
            if let Some(prev_node) = &mut prev{
                // then make (n-1).next = None
                prev_node.borrow_mut().next = None;
            }

            // if n.th node is the only node
            if prev.is_none(){
                self.head = None;
            }

            self.tail = prev;
            return Some(key)
        }
        return None
        // else tail was anyway pointing to Nothing, so leave it None only
    }
}
