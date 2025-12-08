use crate::solution::Solution;
use crate::test_solution;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JunctionBox {
    id: usize,
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct CircuitManager {
    junction_box_to_circuit_id: Vec<usize>,
    circuits: HashMap<usize, Vec<JunctionBox>>,
}

impl CircuitManager {
    fn new(junction_boxes: Vec<JunctionBox>) -> Self {
        let count = junction_boxes.len();
        let mut junction_box_to_circuit_id = Vec::with_capacity(count);
        let mut circuits = HashMap::with_capacity(count);

        for junction_box in junction_boxes {
            junction_box_to_circuit_id.push(junction_box.id);
            circuits.insert(junction_box.id, vec![junction_box]);
        }

        CircuitManager {
            junction_box_to_circuit_id,
            circuits,
        }
    }

    fn connect(&mut self, id_a: usize, id_b: usize) -> bool {
        let circuit_a_id = self.junction_box_to_circuit_id[id_a];
        let circuit_b_id = self.junction_box_to_circuit_id[id_b];

        if circuit_a_id == circuit_b_id {
            return false;
        }

        let boxes_to_move = self.circuits.remove(&circuit_a_id).unwrap();

        for junction_box in &boxes_to_move {
            self.junction_box_to_circuit_id[junction_box.id] = circuit_b_id;
        }

        self.circuits
            .get_mut(&circuit_b_id)
            .unwrap()
            .extend(boxes_to_move);

        true
    }

    fn get_circuit_sizes(&self) -> Vec<usize> {
        self.circuits
            .values()
            .map(|circuit| circuit.len())
            .collect()
    }

    fn circuit_count(&self) -> usize {
        self.circuits.len()
    }
}

#[derive(Debug)]
pub struct Day08 {
    junction_boxes: Vec<JunctionBox>,
}

impl Day08 {
    fn get_sorted_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();

        for i in 0..self.junction_boxes.len() {
            for j in (i + 1)..self.junction_boxes.len() {
                let distance = self.junction_boxes[i].distance_to(&self.junction_boxes[j]);
                pairs.push((
                    distance,
                    self.junction_boxes[i].id,
                    self.junction_boxes[j].id,
                ));
            }
        }

        pairs.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        pairs
            .iter()
            .map(|p| (p.1, p.2))
            .collect::<Vec<(usize, usize)>>() // no need to keep the distance
    }
}

impl Solution for Day08 {
    fn new(input: &str) -> Self {
        let junction_boxes = input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let nums: Vec<i32> = line.split(',').map(|n| n.trim().parse().unwrap()).collect();
                JunctionBox {
                    id,
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                }
            })
            .collect();

        Day08 { junction_boxes }
    }

    fn part_1(&self) -> String {
        let mut circuits = CircuitManager::new(self.junction_boxes.clone());
        let pairs = self.get_sorted_pairs();

        for (id_a, id_b) in pairs.iter().take(1000) {
            circuits.connect(*id_a, *id_b);
        }

        let mut sizes = circuits.get_circuit_sizes();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        sizes.iter().take(3).product::<usize>().to_string()
    }

    fn part_2(&self) -> String {
        let mut circuits = CircuitManager::new(self.junction_boxes.clone());
        let pairs = self.get_sorted_pairs();

        for (id_a, id_b) in pairs {
            if circuits.connect(id_a, id_b) && circuits.circuit_count() == 1 {
                let box_a = &self.junction_boxes[id_a];
                let box_b = &self.junction_boxes[id_b];

                let result = box_a.x * box_b.x;
                return result.to_string();
            }
        }

        panic!("Shouldn't happen!");
    }
}

test_solution!(8, "20", "25272");
