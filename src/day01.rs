use crate::solution::Solution;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
pub struct Day01 {
    rotations: Vec<Rotation>,
}

impl Day01 {
    const INITIAL_POSITION: i32 = 50;
    const DIAL_SIZE: i32 = 100;
}

impl Solution for Day01 {
    fn new(input: &str) -> Self {
        let rotations = input
            .lines()
            .map(|line| {
                let dir_char = line.chars().next().unwrap();
                let distance: i32 = line[1..].parse().unwrap();

                let direction = match dir_char {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(), // valid input, this should never happen
                };

                Rotation {
                    direction,
                    distance,
                }
            })
            .collect();

        Day01 { rotations }
    }

    fn part_1(&self) -> String {
        let mut dial_position = Day01::INITIAL_POSITION;
        let mut zero_hits_at_end = 0;

        for rotation in &self.rotations {
            let displacement = match rotation.direction {
                Direction::Left => -rotation.distance,
                Direction::Right => rotation.distance,
            };

            dial_position = (dial_position + displacement).rem_euclid(Day01::DIAL_SIZE);

            if dial_position == 0 {
                zero_hits_at_end += 1;
            }
        }

        zero_hits_at_end.to_string()
    }

    fn part_2(&self) -> String {
        let mut dial_position = Day01::INITIAL_POSITION;
        let mut zero_hits_per_click = 0;

        for rotation in &self.rotations {
            let click_unit = match rotation.direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };

            for _ in 0..rotation.distance {
                dial_position = (dial_position + click_unit).rem_euclid(Day01::DIAL_SIZE);

                if dial_position == 0 {
                    zero_hits_per_click += 1;
                }
            }
        }

        zero_hits_per_click.to_string()
    }
}

#[test]
fn test_day01() {
    let test_input = fs::read_to_string("input/day01_test.txt").unwrap();
    let day01 = Day01::new(&*test_input);

    assert_eq!(day01.part_1(), "3");
    assert_eq!(day01.part_2(), "6");
}
