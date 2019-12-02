mod day01;
mod day02;

static DAY01_INPUT: &'static str = include_str!("day01.txt");
static DAY02_INPUT: &'static str = include_str!("day02.txt");

fn main() {
    let answer_1a = day01::part_1(DAY01_INPUT);
    println!("Day 1 - Part 1 {:?}", answer_1a);
    let answer_1b = day01::part_2(DAY01_INPUT);
    println!("Day 1 - Part 2 {:?}", answer_1b);

    let answer_2a = day02::part_1(DAY02_INPUT);
    println!("Day 2 - Part 1 {:?}", answer_2a);
    let answer_2b = day02::part_2(DAY02_INPUT);
    println!("Day 2 - Part 2 {:?}", answer_2b);
}
