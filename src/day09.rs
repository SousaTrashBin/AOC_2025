use crate::day04::Position;
use crate::solution::Solution;
use crate::test_solution;

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rectangle {
    fn from_corners(p1: Position, p2: Position) -> Self {
        Self {
            min_x: p1.x.min(p2.x),
            max_x: p1.x.max(p2.x),
            min_y: p1.y.min(p2.y),
            max_y: p1.y.max(p2.y),
        }
    }

    fn area(&self) -> i64 {
        let width = (self.max_x - self.min_x).abs() as i64 + 1;
        let height = (self.max_y - self.min_y).abs() as i64 + 1;
        width * height
    }

    fn contains_point(&self, p: Position) -> bool {
        p.x > self.min_x && p.x < self.max_x && p.y > self.min_y && p.y < self.max_y
    }

    fn is_sliced_by_vertical_edge(&self, start: Position, end: Position) -> bool {
        start.x == end.x
            && start.x > self.min_x
            && start.x < self.max_x
            && start.y.min(end.y) <= self.min_y
            && start.y.max(end.y) >= self.max_y
    }

    fn is_sliced_by_horizontal_edge(&self, start: Position, end: Position) -> bool {
        start.y == end.y
            && start.y > self.min_y
            && start.y < self.max_y
            && start.x.min(end.x) <= self.min_x
            && start.x.max(end.x) >= self.max_x
    }
}

#[derive(Debug)]
pub struct Day09 {
    tiles: Vec<Position>,
}

impl Day09 {
    fn has_tile_inside(&self, rect: &Rectangle) -> bool {
        self.tiles.iter().any(|&p| rect.contains_point(p))
    }

    fn is_sliced_by_edge(&self, rect: &Rectangle) -> bool {
        (0..self.tiles.len()).any(|k| {
            let start = self.tiles[k];
            let end = self.tiles[(k + 1) % self.tiles.len()];

            rect.is_sliced_by_vertical_edge(start, end)
                || rect.is_sliced_by_horizontal_edge(start, end)
        })
    }
}

impl Solution for Day09 {
    fn new(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Position {
                    x: x.trim().parse().unwrap(),
                    y: y.trim().parse().unwrap(),
                }
            })
            .collect();
        Day09 { tiles }
    }

    fn part_1(&self) -> String {
        let mut max_area = 0;
        for (i, &p1) in self.tiles.iter().enumerate() {
            for &p2 in &self.tiles[i + 1..] {
                let rect = Rectangle::from_corners(p1, p2);
                max_area = max_area.max(rect.area());
            }
        }
        max_area.to_string()
    }

    fn part_2(&self) -> String {
        let mut max_area = 0;

        for i in 0..self.tiles.len() {
            for j in (i + 1)..self.tiles.len() {
                let candidate = Rectangle::from_corners(self.tiles[i], self.tiles[j]);

                if candidate.area() <= max_area
                    || self.has_tile_inside(&candidate)
                    || self.is_sliced_by_edge(&candidate)
                {
                    continue;
                }

                max_area = candidate.area();
            }
        }
        max_area.to_string()
    }
}

test_solution!(9, "50", "24");
