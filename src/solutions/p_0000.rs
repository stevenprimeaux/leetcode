use crate::{
    constants::{PARENS, ROMAN},
    util::{deoverflow, is_closing, is_opening, reverse},
    ListNode, Solution,
};

use std::{
    cmp::{max, min},
    collections::HashMap,
    convert::TryInto,
    iter::zip,
};

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

        deoverflow(&mut digits_update, 10, false);

        digits_update
    }

    // 67
    pub fn add_binary(a: String, b: String) -> String {
        let l: usize = max(a.len(), b.len());

        let a_array: Vec<i32> = format!("{:0>width$}", a, width = l)
            .split("")
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        let b_array: Vec<i32> = format!("{:0>width$}", b, width = l)
            .split("")
            .filter_map(|s: &str| s.parse().ok())
            .collect();

        let mut sums: Vec<i32> = zip(a_array, b_array).rev().map(|(a, b)| a + b).collect();

        deoverflow(&mut sums, 2, true);

        sums.iter().rev().map(|i: &i32| i.to_string()).collect()
    }

    // 69
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low: i32 = 2;
        let mut high: i32 = min(x / 2, 46340);

        if x == 0 {
            return 0;
        }
        if x <= 3 {
            return 1;
        }
        if x >= high * high {
            return high;
        }

        let mut root: i32;
        let mut square: i32;
        while high - low > 1 {
            root = (low + high) / 2;
            square = root * root;
            match square.cmp(&x) {
                std::cmp::Ordering::Equal => return root,
                std::cmp::Ordering::Less => low = root,
                std::cmp::Ordering::Greater => high = root,
            }
        }

        low
    }

    // 70
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        let mut c: i32 = 0;
        let mut count: i32 = 0;

        while count < n {
            c = a + b;
            a = b;
            b = c;
            count += 1;
        }

        c
    }

    // 83
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node_current) => match node_current.next {
                None => Some(Box::new(ListNode {
                    val: node_current.val,
                    next: None,
                })),
                Some(node_next) => match node_current.val == node_next.val {
                    true => Self::delete_duplicates(Some(node_next)),
                    false => Some(Box::new(ListNode {
                        val: node_current.val,
                        next: Self::delete_duplicates(Some(node_next)),
                    })),
                },
            },
        }
    }
}
