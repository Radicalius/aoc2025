use crate::solution::Solution;

struct Tile {
  x: i64,
  y: i64
}

impl Tile {
  fn area(&self, other: &Tile) -> i64 {
    return (other.x - self.x + 1).abs() * (other.y - self.y + 1).abs() 
  }
}

pub struct Day9Solution {}

impl Day9Solution {
  fn parse(&self, input: &str) -> Vec<Tile> {
    let mut res = vec![];

    for line in input.split("\n") {
      let parts = line.split(",").collect::<Vec<&str>>();
      if parts.len() != 2 {
        continue;
      }

      let x = parts[0].parse::<i64>().expect(format!("invalid x coord in {}", line).as_str());
      let y = parts[1].parse::<i64>().expect(format!("invalid x coord in {}", line).as_str());

      res.push(Tile { x, y });
    }

    return res;
  }
}

impl Solution for Day9Solution {
  fn part1(&self, input: &str) -> i64 {
    let tiles = self.parse(input);

    let mut max_area = 0;
    for a in &tiles {
      for b in &tiles {
        if a.area(b) > max_area {
          max_area = a.area(b);
        }
      }
    }

    return max_area;
  }

  fn part2(&self, _: &str) -> i64 {
    return 0;
  }
}

