use crate::day09::Day09;
use crate::solution::Solution;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day09.txt").unwrap();
    let day = Day09::new(&input);

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

#[macro_export]
macro_rules! test_solution {
    ($day:expr, $expected1:expr, $expected2:expr) => {
        paste::paste! {
            #[test]
            fn [<test_day$day>]() {
                use crate::solution::Solution;
                use std::fs;

                let input = fs::read_to_string(concat!("input/day0", $day, "_test.txt")).unwrap();
                let solver = [<Day0$day>]::new(&input);
                println!("{:?}", solver);

                assert_eq!(solver.part_1(), $expected1);
                assert_eq!(solver.part_2(), $expected2);
            }
        }
    };
}
