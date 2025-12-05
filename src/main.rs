use crate::day05::Day05;
use crate::solution::Solution;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day05.txt").unwrap();
    let day = Day05::new(&input);

    let measure = |f: &dyn Fn() -> String| {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed().as_millis();
        (result, elapsed)
    };

    let (p1, t1) = measure(&|| day.part_1());
    println!("part 1: {} ({} ms)", p1, t1);

    let (p2, t2) = measure(&|| day.part_2());
    println!("part 2: {} ({} ms)", p2, t2);
}
