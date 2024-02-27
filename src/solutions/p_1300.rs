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
}
