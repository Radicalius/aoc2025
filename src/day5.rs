use std::cmp::Ordering;

use crate::solution::Solution;

struct IngredientRange {
  start: i64,
  end: i64
}

impl IngredientRange {
  fn parse(input: &str) -> IngredientRange {
    let parts = input.split("-").collect::<Vec<&str>>();
    assert!(parts.len() == 2, "invalid ingredient range {}", input);

    let start = parts[0].parse::<i64>().expect("invalid start pf ingredient range");
    let end = parts[1].parse::<i64>().expect("invalid end pf ingredient range");

    return IngredientRange { start, end };
  }

  fn is_inside(&self, inp: i64) -> bool {
    return inp >= self.start && inp <= self.end;
  }
}

struct IngredientInput {
  fresh: Vec<IngredientRange>,
  available: Vec<i64>
}

impl IngredientInput {
  fn parse(input: &str) -> IngredientInput {
    let mut fresh: Vec<IngredientRange> = vec![];
    let mut available: Vec<i64> = vec![];

    for row in input.split("\n") {
      if row == "" {
        continue;
      } else if row.contains("-") {
        fresh.push(IngredientRange::parse(row));
      } else {
        let ingredient = row.parse::<i64>();
        if ingredient.is_ok() {
          available.push(ingredient.unwrap());
        }
      }
    }

    return IngredientInput { fresh: fresh, available: available }
  }
}

pub struct Day5Solution {}

impl Solution for Day5Solution {
  fn part1(&self, input: &str) -> i64 {
    let inp = IngredientInput::parse(input);
    
    let mut fresh = 0;
    for ing in inp.available {
      for range in &inp.fresh {
        if range.is_inside(ing) {
          fresh += 1;
          break;
        }
      }
    }

    return fresh;
  }

  fn part2(&self, input: &str) -> i64 {
    let mut inp = IngredientInput::parse(input);

    inp.fresh.sort_by(|a, b| {
      if a.start < b.start {
        return Ordering::Less;
      } else if a.start > b.start {
        return Ordering::Greater;
      } else {
        return Ordering::Equal;
      }
    });

    let mut total = 0;
    let mut max = 0;

    for range in inp.fresh {

      if range.start <= max && range.end <= max {
        continue;
      } else if range.start <= max {
        total += range.end - max;
        max = range.end;
      } else {
        total += range.end - range.start + 1;
        max = range.end;
      }
    }

    return total;
  }
}