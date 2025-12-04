use crate::solution::Solution;

pub struct Day3Solution {}

impl Day3Solution {
  fn parse(&self, inp: &str) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = vec![];

    for line in inp.split("\n") {
      if line == "" {
        continue;
      }

      let mut bank: Vec<u32> = vec![];
      for battery in line.chars().collect::<Vec<char>>() {
        let joltage = match battery.to_string().parse::<u32>() {
          Ok(x) => x,
          Err(e) => panic!("invalid joltage: {} {}", battery, e)
        };

        bank.push(joltage);
      }

      res.push(bank);
    }

    return res;
  }

  fn find_max_joltage(&self, bank: Vec<u32>) -> u32 {
    let mut cur_max = 0;
    let mut best_bat = 0;

    for i in 0..bank.len() {
      let cur_best = bank[i] + best_bat * 10;
      if cur_best > cur_max {
        cur_max = cur_best;
      }

      if bank[i] > best_bat {
        best_bat = bank[i];
      }
    }

    return cur_max;
  }
}

impl Solution for Day3Solution {
  fn part1(&self, input: &str) -> i64 {
    let mut joltage = 0;    
    for bank in self.parse(input) {
      joltage += self.find_max_joltage(bank)
    }

    return joltage as i64;
  }

  fn part2(&self, input: &str) -> i64 {
    return 0;
  }
}