use crate::{
    constants::{PARENS, ROMAN},
    util::{is_closing, is_opening, reverse},
    Solution,
};

use std::{collections::HashMap, convert::TryInto};

impl Solution {
    // 1
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapping: HashMap<i32, usize> = HashMap::new();

        let mut need: i32;
        for (i_1, v) in nums.iter().enumerate() {
            need = target - v;
            if let Some(&i_2) = mapping.get(&need) {
                return vec![i_1.try_into().unwrap(), i_2.try_into().unwrap()];
            }
            mapping.insert(*v, i_1);
        }

        vec![]
    }

    // 9
    pub fn is_palindrome(x: i32) -> bool {
        x == reverse(x)
    }

    // 13
    pub fn roman_to_int(s: String) -> i32 {
        let mut num_roman: String = s;
        let mut num_int: i32 = 0;

        let mut count_current: i32;
        for (r, val) in ROMAN {
            count_current = num_roman.matches(r).count().try_into().unwrap();
            num_int += val * count_current;

            num_roman = num_roman.replace(r, "");
        }

        num_int
    }

    // 14
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common: String = "".to_string();

        if let Some(first) = strs.get(0) {
            for (i, c) in first.char_indices() {
                for str in &strs {
                    match str.chars().nth(i) {
                        Some(c_try) => {
                            if c_try != c {
                                return common;
                            }
                        }
                        None => return common,
                    }
                }
                common.push(c);
            }
        }

        common
    }

    // 20
    pub fn is_valid(s: String) -> bool {
        let opening_for: HashMap<char, char> = HashMap::from(PARENS);

        let mut s_stack: Vec<char> = vec![];
        for c in s.chars() {
            if is_opening(c) {
                s_stack.push(c);
            }
            if is_closing(c) {
                if !s_stack.ends_with(&[*opening_for.get(&c).unwrap()]) {
                    return false;
                }
                s_stack.pop();
            }
        }

        s_stack.is_empty()
    }
}
