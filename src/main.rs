mod day01;

static DAY01_INPUT: &'static str = include_str!("day01.txt");


fn main() {
    let answer_1a = day01::part_1(DAY01_INPUT);
    println!("Part 1 {:?}", answer_1a);
    let answer_1b = day01::part_2(DAY01_INPUT);
    println!("Part 2 {:?}", answer_1b);
}
