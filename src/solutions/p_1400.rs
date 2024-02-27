use crate::Solution;

impl Solution {
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
}
