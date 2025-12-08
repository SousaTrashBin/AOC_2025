use crate::solution::Solution;
use crate::test_solution;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Position {
    pub fn move_by(&self, dx: i32, dy: i32) -> Self {
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaperRoll {
    position: Position,
}

#[derive(Debug)]
pub struct Day04 {
    pub paper_rolls: HashMap<Position, PaperRoll>,
}

impl Solution for Day04 {
    fn new(input: &str) -> Self {
        let paper_rolls = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    (ch == '@').then(|| {
                        let position = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        let paper_roll = PaperRoll { position };
                        (position, paper_roll)
                    })
                })
            })
            .collect::<HashMap<_, _>>();

        Day04 { paper_rolls }
    }

    fn part_1(&self) -> String {
        self.paper_rolls
            .values()
            .filter(|roll| self.can_paper_roll_be_accessed(roll, &self.paper_rolls))
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut paper_rolls = self.paper_rolls.clone();
        let mut removed_count = 0;

        let mut to_be_removed: Vec<_> = paper_rolls
            .values()
            .filter(|roll| self.can_paper_roll_be_accessed(roll, &paper_rolls))
            .cloned()
            .collect();

        while let Some(roll) = to_be_removed.pop() {
            if let Some(_) = paper_rolls.remove(&roll.position) {
                // a roll can be added twice
                removed_count += 1;

                for neighbor_pos in roll.position.neighbours() {
                    if let Some(neighbor_roll) = paper_rolls.get(&neighbor_pos)
                        && self.can_paper_roll_be_accessed(neighbor_roll, &paper_rolls)
                    {
                        to_be_removed.push(neighbor_roll.clone());
                    }
                }
            }
        }

        removed_count.to_string()
    }
}

impl Position {
    fn neighbours(&self) -> impl Iterator<Item = Position> {
        const DIRS: [(i32, i32); 8] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        DIRS.into_iter().map(move |(dx, dy)| Position {
            x: self.x + dx,
            y: self.y + dy,
        })
    }
}

impl Day04 {
    fn can_paper_roll_be_accessed(
        &self,
        paper_roll: &PaperRoll,
        map: &HashMap<Position, PaperRoll>,
    ) -> bool {
        paper_roll
            .position
            .neighbours()
            .filter(|pos| map.contains_key(pos))
            .count()
            < 4
    }
}

test_solution!(4, "13", "43");
