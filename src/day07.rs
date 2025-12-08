use crate::day04::Position;
use crate::solution::Solution;
use crate::test_solution;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Day07 {
    start_pos: Position,
    splitter_positions: HashSet<Position>,
    max_y: i32,
}

impl Solution for Day07 {
    fn new(input: &str) -> Self {
        let chars_to_positions = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars().enumerate().map(move |(col_idx, ch)| {
                    (
                        ch,
                        Position {
                            y: row_idx as i32,
                            x: col_idx as i32,
                        },
                    )
                })
            })
            .fold(HashMap::new(), |mut acc, (ch, pos)| {
                acc.entry(ch).or_insert_with(HashSet::new).insert(pos);
                acc
            });

        Day07 {
            start_pos: chars_to_positions
                .get(&'S')
                .and_then(|positions| positions.iter().next())
                .unwrap()
                .clone(),
            splitter_positions: chars_to_positions.get(&'^').unwrap().clone(),
            max_y: input.lines().count() as i32,
        }
    }

    fn part_1(&self) -> String {
        fn go(curr_pos: Position, visited: &mut HashSet<Position>, ctx: &Day07) -> usize {
            if curr_pos.y == ctx.max_y || visited.contains(&curr_pos) {
                return 0;
            }

            visited.insert(curr_pos);

            let next_pos = curr_pos.move_by(0, 1);

            if ctx.splitter_positions.contains(&next_pos) {
                let left_pos = next_pos.move_by(-1, 0);
                let right_pos = next_pos.move_by(1, 0);

                1 + go(left_pos, visited, ctx) + go(right_pos, visited, ctx)
            } else {
                go(next_pos, visited, ctx)
            }
        }

        let mut visited = HashSet::new();
        go(self.start_pos, &mut visited, self).to_string()
    }

    fn part_2(&self) -> String {
        fn go(curr_pos: Position, cache: &mut HashMap<Position, u64>, ctx: &Day07) -> u64 {
            if curr_pos.y == ctx.max_y {
                return 1;
            }
            if let Some(&count) = cache.get(&curr_pos) {
                return count;
            }

            let next_pos = curr_pos.move_by(0, 1);

            let total = if ctx.splitter_positions.contains(&next_pos) {
                let left_pos = next_pos.move_by(-1, 0);
                let right_pos = next_pos.move_by(1, 0);

                go(left_pos, cache, ctx) + go(right_pos, cache, ctx)
            } else {
                go(next_pos, cache, ctx)
            };

            cache.insert(curr_pos, total);
            total
        }

        let mut memo = HashMap::new();
        go(self.start_pos, &mut memo, self).to_string()
    }
}

test_solution!(Day07, "input/day07_test.txt", "21", "40");
