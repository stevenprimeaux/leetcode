pub fn reverse(mut x: i32) -> i32 {
    let mut x_rev: i32 = 0;
    while x > 0 {
        x_rev = (10 * x_rev) + (x % 10);
        x /= 10;
    }

    x_rev
}

pub fn is_opening(c: char) -> bool {
    "({[".chars().any(|op: char| c == op)
}

pub fn is_closing(c: char) -> bool {
    ")}]".chars().any(|cl: char| c == cl)
}
