use std::iter::successors;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|m| m.parse::<usize>().unwrap() / 3 - 2)
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|m| {
            successors((m.parse::<usize>().unwrap() / 3).checked_sub(2), |x| {
                (x / 3).checked_sub(2)
            })
            .sum::<usize>()
        })
        .sum()
}