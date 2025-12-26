use std::cmp::{min, max};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
struct Tile {
  x: i64,
  y: i64
}

impl Tile {
  fn area(&self, other: &Tile) -> i64 {
    return ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1)
  }

  fn inside(&self, a: &Tile, b: &Tile) -> bool {
    return min(a.x, b.x) < self.x && self.x < max(a.x, b.x) &&
           min(a.y,b.y) < self.y && self.y < max(a.y, b.y);
  }
}

#[derive(Debug)]
struct Line {
  t1: Tile,
  t2: Tile,
  center: Tile
}

impl Line {
  fn new(a: &Tile, b: &Tile) -> Line {
    return Line { t1: *a, t2: *b, center: Tile { x: (a.x + (b.x - a.x) / 2), y: (a.y + (b.y - a.y) / 2) } }
  }

  fn contains(&self, x: i64, y: i64) -> bool {
    return (self.t1.x == self.t2.x && x == self.t1.x && y >= min(self.t1.y, self.t2.y) && y <= max(self.t1.y, self.t2.y)) ||
           (self.t1.y == self.t2.y && y == self.t1.y && x >= min(self.t1.x, self.t2.x) && x <= max(self.t1.x, self.t2.x));
  }

  fn intersect_square(&self, a: &Tile, b: &Tile) -> bool {
    return self.t1.inside(a, b) || self.t2.inside(a, b) || self.center.inside(a, b);
  }

  fn intersect_ray(&self, x: i64, y: i64) -> bool {
    let int_x;
    let int_y;
    if self.t1.x == self.t2.x {
      int_x = self.t1.x;
      int_y = y + (self.t1.x - x);
      if int_y < min(self.t1.y, self.t2.y) || int_y > max(self.t1.y, self.t2.y) {
        return false;
      }
    } else {
      int_x = x + (self.t1.y - y);
      int_y = self.t1.y;
      if int_x < min(self.t1.x, self.t2.x) || int_x > max(self.t1.x, self.t2.x) {
        return false;
      }
    }

    return int_x >= x && int_y >= y;
  }
}

pub struct Day9Solution {}

impl Day9Solution {
  fn parse(&self, input: &str) -> Vec<Tile> {
    let mut res = vec![];

    let rem_cr = input.replace("\r", "");
    for line in rem_cr.split("\n") {
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

  fn find_lines(&self, tiles: &Vec<Tile>) -> Vec<Line> {
    let mut lines = Vec::new();
    for i in 1..tiles.len() {
      lines.push(Line::new(&tiles[i-1], &tiles[i]));
    }

    lines.push(Line::new(&tiles[0], &tiles[tiles.len()-1]));

    return lines;
  }

  fn is_inside(&self, lines:&Vec<Line>, x:i64, y:i64) -> bool {
    let mut count = 0;
    for l in lines {
      if l.contains(x, y) {
        return true;
      }

      if l.intersect_ray(x, y) {
        count+=1;
      }
    }
    return count % 2 == 1;
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

  fn part2(&self, input: &str) -> i64 {
    let tiles = self.parse(input);
    let lines = self.find_lines(&tiles);

    let mut max_area = 0;
    for a in &tiles {
      'out: for b in &tiles {
        if a.area(b) > max_area {

          if !self.is_inside(&lines, b.x, a.y) {
            continue;
          }

          if !self.is_inside(&lines, a.x, b.y) {
            continue;
          }

          if !self.is_inside(&lines, a.x + (b.x - a.x) / 2, a.y + (b.y - a.y) / 2) {
            continue;
          }

          for l in &lines {
            if l.intersect_square(a, b) {
              continue 'out;
            }
          }

          max_area = a.area(b);
        }
      }
    }

    return max_area;
  }
}

