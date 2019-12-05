const MIN: usize = 347312;
const MAX: usize = 805915;

pub fn part_1() -> usize {
    let mut password_suitable = Vec::new();
    for x in MIN..MAX + 1 {
        if is_incremental(x) && has_pairs(x) {
            password_suitable.push(x);
        }
    }
    password_suitable.len()
}

pub fn part_2() -> usize {
    let mut password_suitable = Vec::new();
    for x in MIN..MAX + 1 {
        if is_incremental(x) && has_unique_pairs(x) {
            password_suitable.push(x);
        }
    }
    password_suitable.len()
}

fn is_incremental(x: usize) -> bool {
    if x < 10 {
        return true;
    }
    let last_digit = x % 10;
    let second_last_digit = x % 100 / 10;
    return last_digit >= second_last_digit && is_incremental(x / 10);
}

fn has_pairs(x: usize) -> bool {
    if x < 10 {
        return false;
    }
    let last_digit = x % 10;
    let second_last_digit = x % 100 / 10;
    if last_digit == second_last_digit {
        return true;
    } else {
        return has_pairs(x / 10);
    }
}

fn has_unique_pairs(x: usize) -> bool {
    let d1 = x % 10;
    let d2 = x % 100 / 10;
    let d3 = x % 1_000 / 100;
    let d4 = x % 10_000 / 1_000;
    let d5 = x % 100_000 / 10_000;
    let d6 = x % 1_000_000 / 100_000;

    return d1 == d2 && d2 != d3
        || d1 != d2 && d2 == d3 && d3 != d4
        || d2 != d3 && d3 == d4 && d4 != d5
        || d3 != d4 && d4 == d5 && d5 != d6
        || d4 != d5 && d5 == d6;
}
