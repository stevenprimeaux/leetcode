use crate::{ListNode, Solution};

impl Solution {
    // 876
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length: i32 = 0;
        let mut node_current: Box<ListNode> = head.clone()?;
        while node_current.next.is_some() {
            node_current = node_current.next?;
            length += 1;
        }

        node_current = head?;
        for _ in 1..=((length + 1) / 2) {
            node_current = node_current.next?;
        }

        Some(node_current)
    }
}
