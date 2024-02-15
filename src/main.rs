use leetcode::Solution;

fn main() {
    println!("{}", Solution::is_valid("".to_string()));
    println!("{}", Solution::is_valid("()".to_string()));
    println!("{}", Solution::is_valid("()[]{}".to_string()));
    println!("{}", Solution::is_valid("(]".to_string()));
    println!("{}", Solution::is_valid("[".to_string()));
    println!("{}", Solution::is_valid("([]".to_string()));
    println!("{}", Solution::is_valid("]".to_string()));
    println!("{}", Solution::is_valid("[])".to_string()));
}
