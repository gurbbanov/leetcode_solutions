fn main() {}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recurs(l1, l2, 0)
    }

    pub fn add_two_numbers_recurs(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mem: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), Some(node2)) => {
                let sum = node1.val + node2.val + mem;
                let mem = sum / 10;

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_two_numbers_recurs(node1.next, node2.next, mem),
                }))
            }
            (None, Some(node)) | (Some(node), None) => {
                let sum = node.val + mem;
                let mem = sum / 10;

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_two_numbers_recurs(node.next, None, mem),
                }))
            }
            (None, None) => {
                if mem == 0 {
                    None
                } else {
                    Some(Box::new(ListNode::new(mem)))
                }
            }
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
