use std::{collections::{BinaryHeap, HashSet}, usize};

use crate::solution::Solution;

#[derive(Debug)]
struct Machine {
  target_state: u64,
  target_joltage: Vec<i64>,
  buttons: Vec<u64>,
  buttons_part2: Vec<Vec<usize>>
}

impl Machine {
  fn parse(input: &str) -> Machine {
    let mut mac = Machine{ 
      target_state: 0, 
      target_joltage: Vec::new(),
      buttons: vec![],
      buttons_part2: vec![] 
    };
    
    let mut target_offset = 0;
    let mut current_button: u64 = 0;
    let mut current_button_vec: Vec<usize> = vec![];
    let mut current_offset = 0;
    let mut in_button = false;
    let mut in_joltage = false;
    for c in input.chars() {
      match c {
        '#' => {
          mac.target_state = Machine::set(mac.target_state, target_offset);
          target_offset += 1;
        },
        '.' => {
          target_offset += 1;
        }
        '(' => {
          in_button = true;
        }
        ')' => {
          current_button = Machine::set(current_button, current_offset);
          current_button_vec.push(current_offset);
          current_offset = 0;
          mac.buttons.push(current_button);
          current_button = 0;
          mac.buttons_part2.push(current_button_vec);
          current_button_vec = vec![];
          in_button = false;
        },
        '{' => {
          in_joltage = true;
        },
        '}' => {
          mac.target_joltage.push(current_offset as i64);
        },
        ',' => {
          if in_button {
            current_button = Machine::set(current_button, current_offset);
            current_button_vec.push(current_offset);
          }

          if in_joltage {
            mac.target_joltage.push(current_offset as i64);
          }

          current_offset = 0;
        },
        '0'..='9' => {
          current_offset = current_offset * 10 + (c.to_digit(10).unwrap() as usize);
        }
        _ => continue
      }
    }

    return mac;
  }
  
  fn set(input: u64, offset: usize) -> u64 {
    return input | (1 << offset);
  }

  fn press(&self, state: u64, button: usize) -> u64 {
    return state ^ self.buttons[button]
  }
}

struct Matrix {
  contents: Vec<Vec<i64>>
}

impl Matrix {
  fn construct(m: &Machine) -> Matrix {
    let mut matrix = Matrix{ contents: Vec::new() };

    for i in 0..m.target_joltage.len() {
      let mut row = Vec::new();
      for button in 0..m.buttons_part2.len() {
        if m.buttons_part2[button].contains(&i) {
          row.push(1);
        } else {
          row.push(0);
        }
      }

      row.push(m.target_joltage[i]);
      matrix.contents.push(row);
    }

    return matrix;
  }

  fn gaussian_elim(&mut self) {
    for col in 0..self.contents.len() {
      for row in 0..self.contents.len() {
        if self.contents[row][col] == 1 {
          self.swap_rows(row, col);
        }
      }
    }
  }

  fn swap_rows(&mut self, r1: usize, r2: usize) {
    let row1 = self.contents[r1].clone();
    self.contents[r1] = self.contents[r2].clone();
    self.contents[r2] = row1;
  }

  fn print(&self) {
    for i in self.contents.iter() {
      println!("{:?}", i);
    }
  }
}

pub struct Day10Solution {}

impl Day10Solution {
  fn part1_helper(&self, mac: &Machine) -> i64 {
    let mut queue: BinaryHeap<(i64, i64, u64)> = BinaryHeap::new();
    let mut seen: HashSet<u64> = HashSet::new();
    queue.push((0, 0, 0));
    seen.insert(0);
    
    while queue.len() > 0 {
      let top = queue.pop().unwrap();
      let dist = top.1;
      let state = top.2;
      if state == mac.target_state {
        return -dist;
      }

      seen.insert(state);

      for button in 0..mac.buttons.len() {
        let next_state = mac.press(state, button);
        if !seen.contains(&next_state) {
          queue.push((dist - 1 - (next_state ^ mac.target_state).count_ones() as i64, dist - 1, mac.press(state, button)));
        }
      }
    }

    return 0;
  }
}

impl Solution for Day10Solution {
  fn part1(&self, input: &str) -> i64 {
    let mut sum: i64 = 0;
    for line in input.split("\n") {
      let mac = Machine::parse(line);
      let res = self.part1_helper(&mac);
      sum += res as i64;
    }
    return sum;    
  }

  fn part2(&self, input: &str) -> i64 {
    let sum: i64 = 0;
    for line in input.split("\n") {
      let mac = Machine::parse(line);
      println!("{:?}", mac);
      let mut mat = Matrix::construct(&mac);
      mat.print();
      println!();
      mat.gaussian_elim();
      mat.print();
    }
    return sum; 
  }
}
