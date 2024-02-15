use crate::Solution;

use std::collections::HashMap;

impl Solution {
    // 412
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut string_current: String;

        for i in 1..=n {
            string_current = String::new();
            if i % 3 == 0 {
                string_current.push_str("Fizz");
            }
            if i % 5 == 0 {
                string_current.push_str("Buzz");
            }
            if string_current.is_empty() {
                string_current.push_str(&i.to_string());
            }
            result.push(string_current);
        }

        result
    }

    // 451
    pub fn frequency_sort(s: String) -> String {
        let mut freq: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            freq.entry(c).and_modify(|e: &mut u32| *e += 1).or_insert(1);
        }

        let mut freq_vec: Vec<(&char, &u32)> = freq.iter().collect();
        freq_vec.sort_unstable_by_key(|t: &(&char, &u32)| t.1);
        freq_vec.reverse();

        let mut result_string: String = String::new();
        for (k, v) in freq_vec {
            for _i in 0..(*v) {
                result_string.push_str(&k.to_string());
            }
        }

        result_string
    }
}
