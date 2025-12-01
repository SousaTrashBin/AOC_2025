use crate::day01::Day01;
use crate::solution::Solution;
use std::fs;

mod day01;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day1.txt").unwrap();

    let day = Day01::new(&input);

    let (p1, p2) = day.get_solutions();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
