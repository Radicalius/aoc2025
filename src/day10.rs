use std::collections::{BinaryHeap, HashMap, HashSet};

use regex::Regex;

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

  fn press_part2(&self, state: &Vec<i64>, button: usize) -> Vec<i64> {
    let mut new_state = state.clone();
    for i in &self.buttons_part2[button] {
      new_state[*i] += 1;
    }

    return new_state;
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

  fn part2_helper(&self, mac: &Machine) -> i64 {
    let mut queue: BinaryHeap<(i64, i64, Vec<i64>)> = BinaryHeap::new();
    let mut seen: HashSet<String> = HashSet::new();

    let mut init_vec = vec![];
    for _ in 0..mac.target_joltage.len() {
      init_vec.push(0);
    }

    queue.push((0, 0, init_vec.clone()));
    
    while queue.len() > 0 {
      let top = queue.pop().unwrap();
      println!("{:?} {:?}", top, mac.target_joltage);
      let dist = top.1;
      let state = top.2;
      if state == mac.target_joltage {
        return -dist;
      }

      if seen.contains(format!("{:?}", state).as_str()) {
        continue;
      }

      seen.insert(format!("{:?}", state));

      for button in 0..mac.buttons.len() {
        let next_state = mac.press_part2(&state, button);
        if !seen.contains(format!("{:?}", next_state).as_str()) {
          
          let mut valid = true;
          for i in 0..next_state.len() {
            if next_state[i] > mac.target_joltage[i] {
              valid = false;
            }
          }

          if valid {
            queue.push((dist - 1 - Day10Solution::part2_heuristic(&mac.target_joltage, &next_state), dist - 1, next_state));
          }
        }
      }
    }

    return 0;
  }

  fn part2_heuristic(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..a.len() {
      if a != b {
        sum += 1;
      }
    }

    return sum;
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
    let mut sum: i64 = 0;
    for line in input.split("\n") {
      let mac = Machine::parse(line);
      println!("{:?}", mac);
      let res = self.part2_helper(&mac);
      sum += res as i64;
    }
    return sum; 
  }
}
