use leetcode::Solution;

fn main() {
    Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
    Solution::merge(&mut vec![2, 0], 1, &mut vec![1], 1);
    Solution::merge(&mut vec![4, 5, 6, 0, 0, 0], 3, &mut vec![1, 2, 3], 3);
}
