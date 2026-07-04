use crate::Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut curr = &head;

        let mut len = 0;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        len = len - n - 1;

        let mut link = &mut head;
        for _ in 0..len {
            if let Some(node) = link {
                link = &mut node.next;
            }
        }
        let next = link.take();
        let nexts_next = if let Some(nexts_next) = next {
            nexts_next.next
        } else {
            None
        };
        *link = nexts_next;

        head
    }
}
