use std::fmt::Display;
mod some;
fn main() {
    let mut list: List<i32> = List::new();
    let input = vec![1, 2, 3, 4, 5];
    for ele in input {
        list.push_back(ele);
    }
    list.delete_nth_node_from_end(2);

    // Displaying ..
    let mut pointer = &mut list.head;
    while let Some(node) = pointer {
        print!("-->{} ", node.val);
        pointer = &mut node.next
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<LLNode<T>>>,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LLNode<T> {
    pub val: T,
    pub next: Option<Box<LLNode<T>>>,
}

impl<T> LLNode<T> {
    pub fn new(v: T) -> Self {
        Self { val: v, next: None }
    }
}

impl<T: Display + Copy> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, val: T) {
        let mut new_node = Box::new(LLNode::new(val));
        let old_head = self.head.take();
        new_node.next = old_head;
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(LLNode::new(val));
        let mut pointer = &mut self.head;

        while let Some(curr_node) = pointer {
            pointer = &mut curr_node.next;
        }
        *pointer = Some(new_node)
    }

    pub fn delete_nth_node(&mut self, n: usize) {
        let mut link = &mut self.head;
        for _ in 0..n {
            if let Some(node) = link {
                link = &mut node.next;
            } else {
                break;
            }
        }
        let mut next = link.take();
        let next_next = next.take().unwrap().next;
        *link = next_next;
    }

    pub fn delete_nth_node_from_end(&mut self, n: usize) {
        let mut pointer = &self.head;

        let mut counter = 0;
        while let Some(node) = pointer {
            counter += 1;
            pointer = &node.next;
        }

        let mut slow = &mut self.head;
        for _ in 0..(counter - n) {
            // iterate to the prev node we want to delete
            if let Some(node) = slow {
                slow = &mut node.next;
            }
        }
        let mut next = slow.take(); // now slow pointer pointing to exactly the node we want;
        *slow = if let Some(nexts_next) = next.take() {
            nexts_next.next
        } else {
            None
        }
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<LLNode<T>>> = None;
        let mut curr: Option<Box<LLNode<T>>> = self.head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        self.head = prev;
    }
}

///!  you can't actually build a cycle with Box<LLNode<T>>
///? because at Box<T> pointer can have at most 1 owner

// Definition for singly-linked list. Just for cycle detection
// we are using raw-pointer: (*mut)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: std::ptr::null_mut(),
            val,
        }
    }
}
pub struct Solution;
impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut slow = head;
        let mut fast = head;

        unsafe {
            // println!("{:?}", (*slow).next); // prints something like: 0x62ec6ba9bd80
            while !fast.is_null() && !(*fast).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;

                if slow == fast {
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_nth_node_from_end(head: *mut ListNode, n: i32) {
        let mut fast = head;
        let mut slow = head;

        let len = n + 1;
        unsafe {
            for _ in 0..len {
                if !fast.is_null() {
                    fast = (*fast).next;
                }
            }
            while !fast.is_null() {
                fast = (*fast).next;
                slow = (*slow).next;
            }

            let next = (*slow).next;
            if !next.is_null() {
                let nexts_next = (*next).next;
                (*slow).next = nexts_next;
            }
        }
    }
}
