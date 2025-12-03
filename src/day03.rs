use crate::solution::Solution;
use std::fs;

#[derive(Debug)]
pub struct Bank {
    batteries: Vec<i64>,
}

#[derive(Debug)]
pub struct Day03 {
    banks: Vec<Bank>,
}

impl Solution for Day03 {
    fn new(input: &str) -> Self {
        let banks = input
            .lines()
            .map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as i64))
            .map(|digits| digits.collect())
            .map(|batteries| Bank { batteries })
            .collect();
        Day03 { banks }
    }

    fn part_1(&self) -> String {
        self.find_max_output_joltage(2)
    }

    fn part_2(&self) -> String {
        self.find_max_output_joltage(12)
    }
}

impl Day03 {
    fn find_max_output_joltage(&self, battery_count: usize) -> String {
        self.banks
            .iter()
            .map(|bank| Self::find_bank_max_output_joltage(battery_count, bank))
            .sum::<i64>()
            .to_string()
    }

    fn find_bank_max_output_joltage(battery_count: usize, bank: &Bank) -> i64 {
        (0..battery_count)
            .fold((0, 0), |(joltage, start_idx), batteries_left| {
                let search_end = bank.batteries.len() - (battery_count - 1 - batteries_left);
                let (offset, &digit) = bank.batteries[start_idx..search_end]
                    .iter()
                    .enumerate()
                    // max by the joltage then the smallest index
                    .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
                    .unwrap();

                (joltage * 10 + digit, start_idx + offset + 1)
            })
            .0
    }
}

#[test]
fn test_day03() {
    let test_input = fs::read_to_string("input/day03_test.txt").unwrap();
    let day03 = Day03::new(&*test_input);

    assert_eq!(day03.part_1(), "357");
    assert_eq!(day03.part_2(), "3121910778619");
}
