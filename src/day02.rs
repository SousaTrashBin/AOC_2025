use crate::solution::Solution;
use crate::test_solution;

#[derive(Debug)]
struct ProductIdRange {
    first_id: i64,
    last_id: i64,
}

#[derive(Debug)]
pub struct Day02 {
    id_ranges_to_verify: Vec<ProductIdRange>,
}

impl Solution for Day02 {
    fn new(input: &str) -> Self {
        let id_ranges_to_verify = input
            .split(',')
            .map(|range| {
                let (first_id_str, last_id_str) = range.split_once('-').unwrap();
                ProductIdRange {
                    first_id: first_id_str.parse().unwrap(),
                    last_id: last_id_str.parse().unwrap(),
                }
            })
            .collect();

        Day02 {
            id_ranges_to_verify,
        }
    }

    fn part_1(&self) -> String {
        let result: i64 = self
            .id_ranges_to_verify
            .iter()
            .flat_map(|range| range.first_id..=range.last_id)
            .filter(|id| {
                let id_str = id.to_string();
                let mid = id_str.len() / 2;
                id_str.len() % 2 == 0 && &id_str[..mid] == &id_str[mid..]
            })
            .sum();

        result.to_string()
    }

    fn part_2(&self) -> String {
        let result: i64 = self
            .id_ranges_to_verify
            .iter()
            .flat_map(|range| range.first_id..=range.last_id)
            .filter(|id| {
                let s = id.to_string();
                let chars = s.chars().collect::<Vec<_>>();

                (1..=chars.len() / 2)
                    .filter(|&block_size| chars.len() % block_size == 0)
                    .any(|block_size| {
                        let block = &chars[..block_size];
                        chars.chunks(block_size).all(|chunk| chunk == block)
                    })
            })
            .sum();

        result.to_string()
    }
}

test_solution!(2, "1227775554", "4174379265");
