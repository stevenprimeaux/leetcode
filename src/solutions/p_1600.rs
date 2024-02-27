use crate::Solution;

impl Solution {
    // 1672
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|customer: &Vec<i32>| customer.iter().sum())
            .max()
            .unwrap()
    }
}
