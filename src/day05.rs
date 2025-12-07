use crate::solution::Solution;
use std::cmp::Ordering;
use std::fs;

type Ingredient = i64;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Range {
    min: Ingredient,
    max: Ingredient,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.min.cmp(&other.min) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Day05 {
    fresh_ingredient_ranges: Vec<Range>,
    ingredients: Vec<Ingredient>,
}

impl Solution for Day05 {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let mut fresh_ingredient_ranges: Vec<_> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split("-");
                let min = parts.next().unwrap().parse::<Ingredient>().unwrap();
                let max = parts.next().unwrap().parse::<Ingredient>().unwrap();
                Range { min, max }
            })
            .collect();
        fresh_ingredient_ranges.sort();

        let mut ingredients: Vec<_> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse::<Ingredient>().unwrap())
            .collect();
        ingredients.sort();

        Day05 {
            fresh_ingredient_ranges,
            ingredients,
        }
    }

    fn part_1(&self) -> String {
        let ranges = &self.fresh_ingredient_ranges;

        let mut fresh_count = 0;
        let mut range_idx = 0;

        for ingredient in &self.ingredients {
            while range_idx < ranges.len() && ingredient > &ranges[range_idx].max {
                range_idx += 1;
            }

            if range_idx >= ranges.len() {
                break;
            }

            if ranges[range_idx].min <= *ingredient {
                fresh_count += 1;
            }
        }

        fresh_count.to_string()
    }

    fn part_2(&self) -> String {
        let mut disjoint_ranges: Vec<Range> = Vec::new();

        for current_range in &self.fresh_ingredient_ranges {
            // checking last is enough due to sorting
            if let Some(last_range) = disjoint_ranges.last_mut() {
                if last_range.is_overlapping(&current_range) {
                    last_range.merge(&current_range);
                    continue;
                }
            }
            disjoint_ranges.push(current_range.clone());
        }

        disjoint_ranges
            .iter()
            .map(|range| range.max - range.min + 1)
            .sum::<i64>()
            .to_string()
    }
}

impl Range {
    pub fn is_overlapping(&self, other: &Range) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    pub fn merge(&mut self, other: &Range) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

#[test]
fn test_day05() {
    let test_input = fs::read_to_string("input/day05_test.txt").unwrap();
    let day05 = Day05::new(&*test_input);
    println!("{:?}", day05);

    assert_eq!(day05.part_1(), "3");
    assert_eq!(day05.part_2(), "14");
}
