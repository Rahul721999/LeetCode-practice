use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut dll: DLL<i32> = DLL::new();
    dll.push_front(0);
    dll.push_front(1);
    dll.push_back(2);
    dll.push_back(3);
    dll.display_forward();
    println!();
    dll.display_backward();

    println!();
    dll.delete_node(3);

    dll.display_forward();
    println!();
    dll.display_backward();
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
// pub type WeakLink<T> = Option<Rc<RefCell<Node<T>>>>; // will use later

/// Doubly linked list NODE, pointing to the Prev and Next node.
#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(v: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            val: v,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
pub struct DLL<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DLL<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn display_forward(&self)
    where
        T: std::fmt::Debug,
    {
        let mut ptr = self.head.clone();

        print!("HEAD: ");
        while let Some(node) = ptr {
            print!("--> {:?}", node.borrow().val);
            ptr = node.borrow().next.clone();
        }
    }

    pub fn display_backward(&self)
    where
        T: std::fmt::Debug,
    {
        let mut ptr = self.tail.clone();

        print!("TAIL:");
        while let Some(node) = ptr {
            print!("--> {:?}", node.borrow().val);
            ptr = node.borrow().prev.clone();
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Node::new(val);
        let old_head = self.head.take(); // take whatever the head was pointing to

        // the node head was pointing previously now should point backwards to the new node
        if let Some(ref old_first_node) = old_head {
            old_first_node.borrow_mut().prev = Some(Rc::clone(&new_node));
        } else {
            // means list was empty, new-node is tail now
            self.tail = Some(Rc::clone(&new_node));
        }

        // new-nodes next should point to the old_head
        new_node.borrow_mut().next = old_head;

        // and now head points to the new-node
        self.head = Some(new_node)
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Node::new(val);
        let old_tail = self.tail.take(); // take out the node which tail was pointing to..

        if let Some(ref old_node) = old_tail {
            // check if tail was pointing to something
            old_node.borrow_mut().next = Some(Rc::clone(&new_node)); // old node now should point to new_node
        } else {
            // means the list was empty
            self.head = Some(Rc::clone(&new_node));
        }

        new_node.borrow_mut().prev = old_tail;
        self.tail = Some(Rc::clone(&new_node)); // new node now points to tail
    }

    pub fn delete_node(&mut self, position: usize)
    where
        T: std::fmt::Debug,
    {
        let mut ptr = self.head.clone();
        for _i in 0..position - 1 {
            if let Some(node) = ptr {
                ptr = node.borrow().next.clone();
            }
        }

        let target = ptr.expect("Position out of bound");
        // pointer is now at the node we have to delete
        println!(
            "Deleting node with value: {:?}",
            target.borrow().val
        );

        let (temp_prev, temp_next) = (
            target.borrow().prev.clone(),
            target.borrow().next.clone()
        );

        if let Some(ref node) = temp_prev {
            node.borrow_mut().next = temp_next.clone();
        } else {
            // first node
            self.head = temp_next.clone();
        }

        if let Some(ref node) = temp_next {
            node.borrow_mut().prev = temp_prev.clone();
        } else {
            // last node
            self.tail = temp_prev.clone();
        }
    }
}
