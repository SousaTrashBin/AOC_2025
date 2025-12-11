use crate::solution::Solution;
use crate::test_solution;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct Day11 {
    devices: HashMap<String, Vec<String>>,
}

impl Day11 {
    fn count_paths_satisfying_requirements<'a>(
        &'a self,
        current: &'a str,
        end: &str,
        requirements: BTreeMap<&'a str, bool>, // needed to use BTreeMap since it needs to be hashable
        memo: &mut HashMap<(&'a str, BTreeMap<&'a str, bool>), u64>, // (device, requirements) : count
    ) -> u64 {
        let mut new_requirements = requirements.clone();
        if let Some(visited) = new_requirements.get_mut(current) {
            *visited = true;
        }

        if current == end {
            return if new_requirements.values().all(|&v| v) {
                1
            } else {
                0
            };
        }

        if let Some(&count) = memo.get(&(current, new_requirements.clone())) {
            return count;
        }

        let mut total_count = 0;
        if let Some(neighbors) = self.devices.get(current) {
            for neighbor in neighbors {
                total_count += self.count_paths_satisfying_requirements(
                    neighbor,
                    end,
                    new_requirements.clone(),
                    memo,
                );
            }
        }

        memo.insert((current, new_requirements), total_count);
        total_count
    }
}

impl Solution for Day11 {
    fn new(input: &str) -> Self {
        let devices = input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let device = parts.next().unwrap().to_string();
                let connections = parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect();
                (device, connections)
            })
            .collect();
        Day11 { devices }
    }

    fn part_1(&self) -> String {
        let mut memo = HashMap::new();
        let requirements = BTreeMap::new();

        self.count_paths_satisfying_requirements("you", "out", requirements, &mut memo)
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut memo = HashMap::new();
        let requirements = BTreeMap::from([("dac", false), ("fft", false)]);

        self.count_paths_satisfying_requirements("svr", "out", requirements, &mut memo)
            .to_string()
    }
}

test_solution!(11, "5", "2");
