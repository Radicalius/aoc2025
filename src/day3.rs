use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day3Solution {}

impl Day3Solution {
  fn parse(&self, inp: &str) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = vec![];

    for line in inp.split("\n") {
      if line == "" {
        continue;
      }

      let mut bank: Vec<i64> = vec![];
      for battery in line.chars().collect::<Vec<char>>() {
        let joltage = match battery.to_string().parse::<i64>() {
          Ok(x) => x,
          Err(e) => panic!("invalid joltage: {} {}", battery, e)
        };

        bank.push(joltage);
      }

      res.push(bank);
    }

    return res;
  }

  fn find_max_joltage_2_battery(&self, bank: &[i64]) -> i64 {
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

  fn find_max_joltage_n_batteries(&self, bank: &[i64], n: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
    // let cache_key = (bank.len(), n);
    // if cache.contains_key(&cache_key) {
    //   return *cache.get(&cache_key).unwrap();
    // }
    
    if n == 0 {
      return 0;
    }

    if bank.len() == 1 {
      return bank[0];
    }
    
    if bank.len() <= n {
      return bank[0] * 10_i64.pow((n - 1) as u32) + self.find_max_joltage_n_batteries(&bank[1..], n-1, cache);
    }

    let use_first = bank[0] * 10_i64.pow((n - 1) as u32) + self.find_max_joltage_n_batteries(&bank[1..], n-1, cache);
    let not_use_first = self.find_max_joltage_n_batteries(&bank[1..], n, cache);

    if use_first > not_use_first {
      //cache.insert(cache_key, use_first);
      return use_first;
    }

    //cache.insert(cache_key, not_use_first);
    return not_use_first;
  }

  fn sum_joltage<JoltageFinder>(&self, inp: Vec<Vec<i64>>, mut find_joltage: JoltageFinder) -> i64 
  where JoltageFinder: FnMut(&[i64]) -> i64 {
    let mut joltage = 0;    
    for bank in inp {
      joltage += find_joltage(&bank)
    }

    return joltage;
  }

}

impl Solution for Day3Solution {
  fn part1(&self, input: &str) -> i64 {
    return self.sum_joltage(
      self.parse(input),
      |bank| self.find_max_joltage_2_battery(bank))
  }

  fn part2(&self, input: &str) -> i64 {
    let mut cache: HashMap<(usize, usize), i64> = HashMap::new();
    return self.sum_joltage(
      self.parse(input),
      |bank| self.find_max_joltage_n_batteries(bank, 12, &mut cache))
  }
}