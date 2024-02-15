pub mod p_0000;
pub mod p_0200;
pub mod p_0300;
pub mod p_0400;

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

    // 1342
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num_current: i32 = num;
        let mut steps: i32 = 0;

        while num_current != 0 {
            if num_current & 1 == 0 {
                num_current >>= 1;
            } else {
                num_current -= 1;
            }
            steps += 1;
        }

        steps
    }

    // 1480
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running: Vec<i32> = vec![];
        let mut sum: i32 = 0;
        for item in nums {
            sum += item;
            running.push(sum);
        }

        running
    }

    // 1672
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|customer: &Vec<i32>| customer.iter().sum())
            .max()
            .unwrap()
    }
}
