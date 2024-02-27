use crate::{
    constants::{PARENS, ROMAN},
    util::{is_closing, is_opening, reverse},
    ListNode, Solution,
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

        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if is_opening(c) {
                stack.push(c);
            }
            if is_closing(c) {
                if !stack.ends_with(&[*opening_for.get(&c).unwrap()]) {
                    return false;
                }
                stack.pop();
            }
        }

        stack.is_empty()
    }

    // 21
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(l_1), Some(l_2)) => match l_1.val < l_2.val {
                true => Some(Box::new(ListNode {
                    val: l_1.val,
                    next: Self::merge_two_lists(l_1.next, Some(l_2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l_2.val,
                    next: Self::merge_two_lists(l_2.next, Some(l_1)),
                })),
            },
        }
    }

    // 26
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();

        nums.len().try_into().unwrap()
    }

    // 27
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x: &i32| x != &val);

        nums.len().try_into().unwrap()
    }

    // 28
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(i) => i.try_into().unwrap(),
            None => -1,
        }
    }

    // 35
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) | Err(i) => i.try_into().unwrap(),
        }
    }

    // 58
    pub fn length_of_last_word(s: String) -> i32 {
        match s.split_whitespace().last() {
            Some(word_last) => word_last.len().try_into().unwrap(),
            None => 0,
        }
    }

    // 66
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_update: Vec<i32> = digits;
        if let Some(last) = digits_update.last_mut() {
            *last += 1;
        }

        digits_update.reverse();
        let mut overflow: bool = true;
        let mut start_current: usize = 0;
        while overflow == true {
            overflow = false;

            for i in start_current..digits_update.len() {
                if digits_update[i] > 9 {
                    if digits_update.get(i + 1).is_none() {
                        digits_update.push(0);
                        overflow = true;
                    }
                    digits_update[i + 1] += digits_update[i] / 10;
                    digits_update[i] %= 10;
                }
            }
            start_current = digits_update.len() - 1;
        }
        digits_update.reverse();

        digits_update
    }
}
