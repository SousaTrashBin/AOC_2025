use crate::solution::Solution;
use std::fmt::{Debug, Formatter};
use std::fs;

struct Problem {
    numbers: Vec<i64>,
    op: Box<dyn Fn(i64, i64) -> i64>,
}

impl Debug for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Problem")
            .field("numbers", &self.numbers)
            .finish()
    }
}

#[derive(Debug)]
pub struct Day06 {
    p1_problems: Vec<Problem>,
    p2_problems: Vec<Problem>,
}

impl Solution for Day06 {
    fn new(input: &str) -> Self {
        Day06 {
            p1_problems: parse_part1(input),
            p2_problems: parse_part2(input),
        }
    }

    fn part_1(&self) -> String {
        self.p1_problems
            .iter()
            .map(|p| p.get_solution())
            .sum::<i64>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.p2_problems
            .iter()
            .map(|p| p.get_solution())
            .sum::<i64>()
            .to_string()
    }
}

impl Problem {
    pub fn get_solution(&self) -> i64 {
        self.numbers
            .iter()
            .skip(1)
            .fold(self.numbers[0], |acc, num| (self.op)(acc, *num))
    }
}

fn parse_operator(op: char) -> Box<dyn Fn(i64, i64) -> i64> {
    match op {
        '*' => Box::new(|a, b| a * b),
        '+' => Box::new(|a, b| a + b),
        _ => panic!("{}", op),
    }
}

fn parse_part1(input: &str) -> Vec<Problem> {
    let mut rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let op_row = rows.pop().expect("missing op");

    (0..op_row.len())
        .map(|col_idx| {
            let op_char = op_row[col_idx].chars().next().unwrap();

            let numbers = rows
                .iter()
                .map(|row| row[col_idx].parse().unwrap())
                .collect();

            Problem {
                numbers,
                op: parse_operator(op_char),
            }
        })
        .collect()
}

fn parse_part2(input: &str) -> Vec<Problem> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // transpose
    let columns: Vec<Vec<char>> = (0..max_width)
        .map(|x| {
            lines
                .iter()
                .map(|line| *line.get(x).unwrap_or(&' '))
                .collect()
        })
        .collect();

    columns
        .split(|col| col.iter().all(|c| c.is_whitespace()))
        .map(|chunk| {
            let op_char = chunk
                .iter()
                .filter_map(|col| col.last())
                .find(|&&c| matches!(c, '+' | '*'))
                .expect("missing op");

            let numbers = chunk
                .iter()
                .rev() // right to left (shouldn't matter due to associativity)
                .filter_map(|col| {
                    let number_str: String = col.iter().filter(|c| c.is_ascii_digit()).collect();
                    number_str.parse().ok()
                })
                .collect();

            Problem {
                numbers,
                op: parse_operator(*op_char),
            }
        })
        .collect()
}

#[test]
fn test_day06() {
    let test_input = fs::read_to_string("input/day06_test.txt").unwrap();
    let day06 = Day06::new(&*test_input);
    println!("{:?}", day06);

    assert_eq!(day06.part_1(), "4277556");
    assert_eq!(day06.part_2(), "3263827");
}
