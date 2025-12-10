use crate::solution::Solution;
use crate::test_solution;
use good_lp::{Solution as LPSolution, SolverModel, coin_cbc, variable, variables};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Light {
    on: bool,
}

impl Light {
    fn from_char(c: char) -> Self {
        Light { on: c == '#' }
    }

    fn toggle(&mut self) {
        self.on = !self.on;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ConfigurationState {
    presses: u64,
    lights: Vec<Light>,
}

impl Ord for ConfigurationState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.presses.cmp(&self.presses)
    }
}

impl PartialOrd for ConfigurationState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Machine {
    light_diagram: Vec<Light>,
    button_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<u64>,
}

impl Machine {
    pub fn from_str(s: &str) -> Self {
        let regex = Regex::new(r"\[([#.]+)]\s(.+)\s\{([\d,]+)}").unwrap();
        let captures = regex.captures(s).unwrap();

        let light_diagram = captures[1].chars().map(Light::from_char).collect();

        let joltage_requirements = captures[3]
            .split(',')
            .map(|jolt_str| jolt_str.parse().unwrap())
            .collect();

        let tuple_regex = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
        let button_schematics = tuple_regex
            .captures_iter(&captures[2])
            .map(|c| {
                c[1].split(',')
                    .map(|btn_schematic_str| btn_schematic_str.parse().unwrap())
                    .collect()
            })
            .collect();

        Machine {
            light_diagram,
            button_schematics,
            joltage_requirements,
        }
    }

    pub(crate) fn min_presses_for_lights(&self) -> u64 {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push(ConfigurationState {
            presses: 0,
            lights: self.light_diagram.clone(),
        });
        visited.insert(self.light_diagram.clone());

        while let Some(ConfigurationState { presses, lights }) = heap.pop() {
            if lights.iter().all(|l| !l.on) {
                return presses;
            }

            for button_schematic in &self.button_schematics {
                let mut next_lights = lights.clone();

                for &idx in button_schematic {
                    if let Some(light) = next_lights.get_mut(idx) {
                        light.toggle();
                    }
                }

                if visited.insert(next_lights.clone()) {
                    heap.push(ConfigurationState {
                        presses: presses + 1,
                        lights: next_lights,
                    });
                }
            }
        }
        0
    }

    pub(crate) fn min_presses_for_joltage(&self) -> u64 {
        let mut vars = variables!();

        let press_vars: Vec<_> = (0..self.button_schematics.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let objective = press_vars.iter().sum::<good_lp::Expression>();

        let mut problem = vars.minimise(objective).using(coin_cbc);

        problem.set_parameter("log", "0");

        for (counter_idx, &target_val) in self.joltage_requirements.iter().enumerate() {
            let mut expression = good_lp::Expression::from(0);

            for (btn_idx, wires) in self.button_schematics.iter().enumerate() {
                if wires.contains(&counter_idx) {
                    expression += press_vars[btn_idx];
                }
            }

            problem.add_constraint(expression.eq(target_val as i32));
        }

        if let Ok(solution) = problem.solve() {
            solution.eval(press_vars.iter().sum::<good_lp::Expression>()) as u64
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Day10 {
    machines: Vec<Machine>,
}

impl Solution for Day10 {
    fn new(input: &str) -> Self {
        let machines = input.lines().map(Machine::from_str).collect();
        Day10 { machines }
    }

    fn part_1(&self) -> String {
        self.machines
            .iter()
            .map(|m| m.min_presses_for_lights())
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.machines
            .iter()
            .map(|m| m.min_presses_for_joltage())
            .sum::<u64>()
            .to_string()
    }
}

test_solution!(10, "7", "33");
