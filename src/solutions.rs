pub mod p_0000;
pub mod p_0200;
pub mod p_0300;
pub mod p_0400;
pub mod p_0800;

use crate::Solution;

impl Solution {
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
