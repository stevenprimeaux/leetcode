pub fn deoverflow(digits: &mut Vec<i32>, base: i32, reversed: bool) {
    if !reversed {
        digits.reverse();
    }

    let mut overflow: bool = true;
    let mut start_current: usize = 0;
    while overflow == true {
        overflow = false;

        for i in start_current..digits.len() {
            if digits[i] >= base {
                if digits.get(i + 1).is_none() {
                    digits.push(0);
                    overflow = true;
                }
                digits[i + 1] += digits[i] / base;
                digits[i] %= base;
            }
        }
        start_current = digits.len() - 1;
    }

    if !reversed {
        digits.reverse();
    }
}

pub fn is_closing(c: char) -> bool {
    ")}]".chars().any(|cl: char| c == cl)
}

pub fn is_opening(c: char) -> bool {
    "({[".chars().any(|op: char| c == op)
}

pub fn reverse(mut x: i32) -> i32 {
    let mut x_rev: i32 = 0;
    while x > 0 {
        x_rev = (10 * x_rev) + (x % 10);
        x /= 10;
    }

    x_rev
}
