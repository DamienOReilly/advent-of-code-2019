mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

static DAY01_INPUT: &'static str = include_str!("day01.txt");
static DAY02_INPUT: &'static str = include_str!("day02.txt");
static DAY03_INPUT: &'static str = include_str!("day03.txt");
static DAY05_INPUT: &'static str = include_str!("day05.txt");

fn main() {
    let answer_1a = day01::part_1(DAY01_INPUT);
    println!("Day 1 - Part 1 {:?}", answer_1a);
    let answer_1b = day01::part_2(DAY01_INPUT);
    println!("Day 1 - Part 2 {:?}", answer_1b);

    let answer_2a = day02::part_1(DAY02_INPUT);
    println!("Day 2 - Part 1 {:?}", answer_2a);
    let answer_2b = day02::part_2(DAY02_INPUT);
    println!("Day 2 - Part 2 {:?}", answer_2b);

    day03::solve(DAY03_INPUT);

    let answer_4a = day04::part_1();
    println!("Day 4 - Part 1 {:?}", answer_4a);
    let answer_4b = day04::part_2();
    println!("Day 4 - Part 2 {:?}", answer_4b);

    let answer_5a = day05::part_1(DAY05_INPUT);
    println!("Day 5 - Part 1 {:?}", answer_5a);
    // let answer_5b = day05::part_2(DAY02_INPUT);
    // println!("Day 5 - Part 2 {:?}", answer_5b);
}
