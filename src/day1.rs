use crate::solution::Solution;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT
}

struct Rotation {
    direction: Direction,
    amount: i32
}

pub struct Day1Solution {}

impl Day1Solution {
  fn parse(&self, input: &str) -> Vec<Rotation> {
    let mut data: Vec<Rotation> = vec![];
    
    for line in input.split("\n") {
      if line == "" {
          continue;
      }

      let dir = line.chars().nth(0).unwrap();
      let direction = match dir {
          'L' => Direction::LEFT,
          'R' => Direction::RIGHT,
          _ => panic!("invalid direction")
      };

      let countStr = line.get(1..).unwrap();
      let count = countStr.parse::<i32>().unwrap();

      data.push(Rotation{
          direction: direction,
          amount: count,
      });
    }

    return data;
  }
}

impl Solution for Day1Solution {
  fn part1(&self, input: &str) -> i64 {
    let data: Vec<Rotation> = self.parse(input);

    let mut cur = 50;
    let mut zeros = 0;

    for rot in data {
      if rot.direction == Direction::LEFT {
          cur = (cur - rot.amount) % 100;
      } else {
          cur = (cur + rot.amount) % 100;
      }

      if cur == 0 {
          zeros += 1
      }
    }

    return zeros;
  }

  fn part2(&self, input: &str) -> i64 {
    let data: Vec<Rotation> = self.parse(input);

    let mut cur = 50;
    let mut zeros = 0;

    for rot in data {

      if rot.direction == Direction::LEFT {
        zeros += (rot.amount / 100) as i64;
        let start = cur;
        cur = cur - (rot.amount % 100);
        if start > 0 && cur <= 0 {
          zeros += 1;
        }

        if cur < 0 {
          cur += 100;
        }
      } else {
        zeros += (rot.amount / 100) as i64;
        let start= cur;
        cur = cur + (rot.amount % 100);
        if start < 100 && cur >= 100 {
          zeros += 1;
        }

        if cur >= 100 {
          cur -= 100;
        }
      }
    }

    return zeros;
  }
}