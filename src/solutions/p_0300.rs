use crate::Solution;

use std::collections::HashMap;

impl Solution {
    // 383
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut letters_have: HashMap<char, u32> = HashMap::new();
        for c in magazine.chars() {
            letters_have
                .entry(c)
                .and_modify(|e: &mut u32| *e += 1)
                .or_insert(1);
        }

        let mut letters_need: HashMap<char, u32> = HashMap::new();
        for c in ransom_note.chars() {
            if !letters_have.contains_key(&c) {
                return false;
            }

            letters_need
                .entry(c)
                .and_modify(|e: &mut u32| *e += 1)
                .or_insert(1);

            if letters_need.get(&c) > letters_have.get(&c) {
                return false;
            }
        }

        true
    }
}
