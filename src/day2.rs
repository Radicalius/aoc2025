
use crate::solution::Solution;

struct IdRange {
  start: i64,
  end: i64
}

pub struct Day2Solution{

}

impl Day2Solution {
  fn parse(&self, input: &str) -> Vec<IdRange> {
    let mut result: Vec<IdRange> = vec![];

    for range in input.split(",") {
      let parts = range.split("-").collect::<Vec<&str>>();
      assert!(parts.len() == 2, "invalid id range");

      let id1 = parts[0].parse::<i64>().expect("invalid id");
      let id2 = parts[1].parse::<i64>().expect("invalid id");

      result.push(IdRange { start: id1, end: id2 });
    }

    return result;
  }

  fn is_invalid(&self, num: i64) -> bool {
    let digits = (num as f64).log10().ceil() as u32;

    if digits % 2 != 0 || digits == 0 {
      return false;
    }

    let half_exp = 10_i64.pow(digits / 2);
    let upper = num / half_exp;
    let lower = num % half_exp;

    return upper == lower;
  }
}

impl Solution for Day2Solution  {
  fn part1(&self, input: &str) -> i64 {
    let data = self.parse(input);

    let mut res: i64 = 0;

    for range in data {
      for i in range.start..(range.end+1) {
        if self.is_invalid(i) {
          res += i;
        }
      }
    }

    return res;
  }

  fn part2(&self, input: &str) -> i64 {
    return 0;
  }
}