use crate::solution::Solution;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
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
        let mut removed_paper_rolls = 0;

        while let Some(pos) = paper_rolls.keys().copied().find(|p| {
            let roll = &paper_rolls[p];
            self.can_paper_roll_be_accessed(roll, &paper_rolls)
        }) {
            paper_rolls.remove(&pos);
            removed_paper_rolls += 1;
        }

        removed_paper_rolls.to_string()
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
        roll: &PaperRoll,
        map: &HashMap<Position, PaperRoll>,
    ) -> bool {
        roll.position
            .neighbours()
            .filter(|pos| map.contains_key(pos))
            .count()
            < 4
    }
}

#[test]
fn test_day04() {
    let test_input = fs::read_to_string("input/day04_test.txt").unwrap();
    let day04 = Day04::new(&*test_input);

    assert_eq!(day04.part_1(), "13");
    assert_eq!(day04.part_2(), "43");
}
