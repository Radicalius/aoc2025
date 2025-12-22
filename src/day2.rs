
use crate::solution::Solution;

struct IdRange {
  start: i64,
  end: i64
}

pub struct Day2Solution{}

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

  fn count_digits(&self, num: i64) -> u32 {
    if num == 0 {
      return 1;
    }

    return num.ilog10() + 1;
  }

  fn get_chunk(&self, num: i64, size: u32, offset: u32) -> i64 {
    return (num / 10_i64.pow(size * offset)) % 10_i64.pow(size)
  }

  fn is_invalid_part2(&self, num: i64) -> bool {
    let digits = self.count_digits(num);
    
    for size in 1..(digits / 2 + 1) {
      if digits % size != 0 {
        continue;
      }

      let first = self.get_chunk(num, size, 0);
      let mut invalid = true;
      for offset in 1..(digits / size) {
        if self.get_chunk(num, size, offset) != first {
          invalid = false;
          break;
        }
      }

      if invalid {
        return true;
      }
    }
    
    return false;
  }

  fn sum_invalid<InvalidChecker>(&self, data: Vec<IdRange>, is_invalid: InvalidChecker) -> i64 where InvalidChecker: Fn(i64) -> bool {
    let mut res: i64 = 0;

    for range in data {
      for i in range.start..(range.end+1) {
        if is_invalid(i) {
          res += i;
        }
      }
    }

    return res;
  }

  fn next_invalid(&self, start: i64) -> i64 {
    let mut res = start;

    let mut digits = self.count_digits(start);
    if digits % 2 != 0 {
      res = 10_i64.pow(digits);
      digits += 1;
    }

    let mut top = self.get_chunk(res, digits / 2, 1);
    res = top + top * 10_i64.pow(digits / 2);
    while res <= start {
      top += 1;
      res = top + top * 10_i64.pow(digits / 2);
      if self.count_digits(res) > digits {
        res = self.next_invalid(res);
      }
    }

    res
  }
}

impl Solution for Day2Solution  {
  fn part1(&self, input: &str) -> i64 {
    let data = self.parse(input);
    let mut sum = 0;
    for range in data {
      let mut cur = self.next_invalid(range.start - 1);
      while cur <= range.end {
        sum += cur;
        cur = self.next_invalid(cur);
      }
    }

    sum
  }

  fn part2(&self, input: &str) -> i64 {
    let data = self.parse(input);
    return self.sum_invalid(data, |data| self.is_invalid_part2(data));
  }
}