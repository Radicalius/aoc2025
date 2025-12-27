use std::{cmp::max, collections::{BinaryHeap, HashSet}, usize};

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
  contents: Vec<Vec<f64>>
}

impl Matrix {
  fn construct(m: &Machine) -> Matrix {
    let mut matrix = Matrix{ contents: Vec::new() };

    for i in 0..m.target_joltage.len() {
      let mut row: Vec<f64> = Vec::new();
      for button in 0..m.buttons_part2.len() {
        if m.buttons_part2[button].contains(&i) {
          row.push(1.0);
        } else {
          row.push(0.0);
        }
      }

      row.push(m.target_joltage[i] as f64);
      matrix.contents.push(row);
    }

    return matrix;
  }

  fn gaussian_elim(&mut self) {
    'col: for col in 0..(self.contents.len()-1) {
      for row in 0..self.contents.len() {
        let mut leading_zeros = 0;
        for i in 0..self.contents[row].len() {
          if self.contents[row][i] == 0.0 {
            leading_zeros += 1;
          } else {
            break;
          }
        }

        if leading_zeros < col {
          continue;
        }

        if self.contents[row][col] != 0.0 {
          self.swap_rows(row, col);
          self.div_row(col, self.contents[col][col] as f64);

          for tar in (col+1)..(self.contents.len()) {
            if self.contents[tar][col] != 0.0 {
              self.sub_rows(col, tar, self.contents[tar][col] as f64);
            }
          }

          continue 'col;
        }
      }
    }
  }

  fn swap_rows(&mut self, r1: usize, r2: usize) {
    let row1 = self.contents[r1].clone();
    self.contents[r1] = self.contents[r2].clone();
    self.contents[r2] = row1;
  }

  fn sub_rows(&mut self, source: usize, target: usize, factor: f64) {
    for i in 0..self.contents[source].len() {
      self.contents[target][i] -= self.contents[source][i] * factor;
    }
  }

  fn div_row(&mut self, tar: usize, factor: f64) {
    for i in 0..self.contents[tar].len() {
      self.contents[tar][i] = self.contents[tar][i] / factor;
    }
  }

  fn find_free_variables(&self) -> Vec<usize> {
    let mut free_variables = vec![];
    for i in 0..(self.contents[0].len() - 1) {
      if self.contents.len() <= i || self.contents[i][i] == 0.0 {
        free_variables.push(i);
      }
    }

    return free_variables;
  }

  fn find_max_value(&self, ind: usize, m_prime: i64) -> f64 {
    let mut m: f64 = 100000000.0;
    let width = self.contents[0].len();
    'out: for row in 0..(self.contents.len()) {
      for i in 0..width {
        if self.contents[row][i] < 0.0 {
          continue 'out;
        }
      }

      if self.contents[row][ind] != 0.0 {
        m = m.min(self.contents[row][width-1] / self.contents[row][ind]);
      }
    }

    return m.min(m_prime as f64);
  }

  fn solve(&self, input: &mut Vec<Option<i64>>) -> bool {
    let mut changed = true;
    while changed {
      changed = false;

      let width = self.contents[0].len();
      'row: for row in 0..self.contents.len() {
        let mut total = self.contents[row][width-1];
        let mut target: Option<usize> = None;
        for col in 0..(width-1) {
          if self.contents[row][col] == 0.0 {
            continue;
          } else if input[col].is_some() {
            total -= input[col].unwrap() as f64 * self.contents[row][col];
          } else if target.is_none() {
            target = Some(col);
          } else {
            continue 'row;
          }
        }

        if target.is_none() && (total > 0.0001 || total < -0.0001) {
          return false;
        }

        if target.is_some() {
          input[target.unwrap()] = Some((total / self.contents[row][target.unwrap()]).round() as i64);
          changed = true;
        }
      }
    }

    return input.iter().all(|x| x.is_some());
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

  fn part2_helper(&self, m: &Matrix, free: &Vec<usize>, maxes: &Vec<f64>, state: &mut Vec<i64>, index: usize) -> Option<i64> {
    if state.len() == free.len() {
      let mut input: Vec<Option<i64>> = vec![];
      for _ in 0..(m.contents[0].len() - 1) {
        input.push(None);
      }

      for (si, j) in free.iter().enumerate() {
        input[*j] = Some(state[si]);
      }

      if m.solve(&mut input) {
        let mut sum = 0;
        for i in &input {
          if i.unwrap() < 0 {
            return None;
          }
          sum += i.unwrap();
        }
        return Some(sum);
      } else {
        return None;
      }
    }

    let mut min: Option<i64> = None;
    for i in 0..(maxes[index] as i64+1) {
      if min.is_some() && i > min.unwrap() {
        break;
      }

      state.push(i);
      let c = self.part2_helper(m, free, maxes, state, index+1);
      if c.is_some() &&  (min.is_none() || c < min) {
        min = c;
      }
      state.pop();
    }

    return min;
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
      let mut mat = Matrix::construct(&mac);
      mat.gaussian_elim();

      let free = mat.find_free_variables();

      let mut m_prime = 0;
      for i in mac.target_joltage {
        m_prime = max(m_prime, i);
      }

      let maxes = free.iter().map(|x| mat.find_max_value(*x, m_prime)).collect::<Vec<f64>>();

      let mut state = vec![];

      let sol = self.part2_helper(&mat, &free, &maxes, &mut state, 0);
      assert!(sol.is_some());

      sum += sol.unwrap();
    }
    return sum; 
  }
}
