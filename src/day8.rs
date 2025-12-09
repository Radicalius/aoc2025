use std::collections::{BinaryHeap, HashSet};

use crate::solution::Solution;

#[derive(PartialEq, Eq, Debug)]
struct JunctionBox {
  x: i64,
  y: i64,
  z: i64,
  connected: Vec<usize>
}

impl JunctionBox {
  fn new(input: &str) -> JunctionBox {
    let parts = input.split(",").collect::<Vec<&str>>();
    assert!(parts.len() == 3, "invalid input for junction box: {}", input);

    let x = parts[0].parse::<i64>().expect(format!("invalid x coord: {}", input).as_str());
    let y = parts[1].parse::<i64>().expect(format!("invalid y coord: {}", input).as_str());
    let z = parts[2].parse::<i64>().expect(format!("invalid z coord: {}", input).as_str());

    return JunctionBox { x, y, z, connected: Vec::new() };
  }

  fn dist(&self, other: &JunctionBox) -> f64 {
    return (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64).sqrt(); 
  }
}

pub struct Day8Solution {}

impl Day8Solution {
  fn parse(&self, input: &str) -> Vec<JunctionBox> {
    return input.split("\n").map(|line| JunctionBox::new(line)).collect::<Vec<JunctionBox>>();
  }

  fn find_circuit_size(&self, boxes: &Vec<JunctionBox>, current: usize, seen: &mut HashSet<usize>) -> i64 {
    if seen.contains(&current) {
      return 0;
    }

    seen.insert(current);

    let mut size = 1;
    for ind in &boxes[current].connected {
      size += self.find_circuit_size(boxes, *ind, seen)
    }

    return size;
  }
}

impl Solution for Day8Solution {
  fn part1(&self, input: &str) -> i64 {
    let mut boxes = self.parse(input);
    let mut sorted: Vec<(usize, usize, f64)> = vec![];
    for a in 0..boxes.len() {
      for b in 0..boxes.len() {
        if a < b {
          sorted.push((a, b, boxes[a].dist(&boxes[b])));
        }
      }
    }

    let mut total_connections = 10;
    if boxes.len() > 20 {
      total_connections = 1000;
    }

    sorted.sort_by(|a, b| a.2.total_cmp(&b.2));
    for connection in 0..total_connections {
      boxes[sorted[connection].0].connected.push(sorted[connection].1);
      boxes[sorted[connection].1].connected.push(sorted[connection].0);
    }

    let mut seen = HashSet::new();
    let mut circuit_sizes = vec![];
    for i in 0..boxes.len() {
      circuit_sizes.push(self.find_circuit_size(&boxes, i, &mut seen));
    }

    circuit_sizes.sort_by(|a,b| b.cmp(a));

    return circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
  }

  fn part2(&self, input: &str) -> i64 {
    return 0;
  }
}