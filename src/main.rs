use crate::day11::Day11;
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
mod day10;
mod day11;
mod solution;

fn main() {
    let input = fs::read_to_string("input/day11.txt").unwrap();
    let day = Day11::new(&input);

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
    (1, $e1:expr, $e2:expr) => {
        $crate::test_solution!(01, $e1, $e2);
    };
    (2, $e1:expr, $e2:expr) => {
        $crate::test_solution!(02, $e1, $e2);
    };
    (3, $e1:expr, $e2:expr) => {
        $crate::test_solution!(03, $e1, $e2);
    };
    (4, $e1:expr, $e2:expr) => {
        $crate::test_solution!(04, $e1, $e2);
    };
    (5, $e1:expr, $e2:expr) => {
        $crate::test_solution!(05, $e1, $e2);
    };
    (6, $e1:expr, $e2:expr) => {
        $crate::test_solution!(06, $e1, $e2);
    };
    (7, $e1:expr, $e2:expr) => {
        $crate::test_solution!(07, $e1, $e2);
    };
    (8, $e1:expr, $e2:expr) => {
        $crate::test_solution!(08, $e1, $e2);
    };
    (9, $e1:expr, $e2:expr) => {
        $crate::test_solution!(09, $e1, $e2);
    };

    ($day:literal, $expected1:expr, $expected2:expr) => {
        paste::paste! {
            #[test]
            fn [<test_day$day>]() {
                use crate::solution::Solution;
                use std::fs;

                let input = fs::read_to_string(
                    concat!("input/day", stringify!($day), "_test.txt")
                ).unwrap();

                let solver = [<Day$day>]::new(&input);
                println!("{:?}", solver);

                assert_eq!(solver.part_1(), $expected1);
                assert_eq!(solver.part_2(), $expected2);
            }
        }
    };
}
