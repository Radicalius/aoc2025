use std::collections::{BinaryHeap, HashMap, HashSet};

use regex::Regex;

use crate::solution::Solution;

#[derive(Debug)]
struct Machine {
  target_state: u64,
  buttons: Vec<u64>
}

impl Machine {
  fn parse(input: &str) -> Machine {
    let mut mac = Machine{ target_state: 0, buttons: vec![] };
    
    let mut target_offset = 0;
    let mut current_button: u64 = 0;
    let mut current_offset = 0;
    let mut in_button = false;
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
          current_offset = 0;
          mac.buttons.push(current_button);
          current_button = 0;
          in_button = false;
        }
        ',' => {
          if in_button {
            current_button = Machine::set(current_button, current_offset);
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
    return 0;
  }
}
