use std::fmt::Debug;

use regex::Regex;

use crate::solution::Solution;

#[derive(PartialEq)]
enum Op {
  INVALID,
  ADD,
  SUBTRACT,
  MULTIPLY,
  DIVIDE
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALID => write!(f, "INVALID"),
            Self::ADD => write!(f, "ADD"),
            Self::SUBTRACT => write!(f, "SUBTRACT"),
            Self::MULTIPLY => write!(f, "MULTIPLY"),
            Self::DIVIDE => write!(f, "DIVIDE"),
        }
    }
}

#[derive(Debug)]
struct MathProblem {
  operands: Vec<i64>,
  operator: Op
}

impl MathProblem {
  fn new() -> MathProblem {
    return MathProblem { operands: Vec::new(), operator: Op::INVALID };
  }

  fn evaluate(&self) -> i64 {
    if self.operands.len() == 0 {
      return 0;
    }

    let mut result = self.operands[0];
    for operand in self.operands[1..].iter() {
      result = match self.operator {
        Op::ADD => result + operand,
        Op::SUBTRACT => result - operand,
        Op::MULTIPLY => result * operand,
        Op::DIVIDE => result / operand,
        _ => panic!("invalid operator")
      }
    }

    return result;
  }
}

pub struct Day6Solution {}

impl Day6Solution {
  fn parse_part1(&self, input: &str) -> Vec<MathProblem> {    
    let delim_regex = Regex::new(r"[^0-9^+^*^/^-]+").expect("invalid delim regex");
    
    let mut problems: Vec<MathProblem> = vec![];

    for row in input.split("\n") {
      let matches = delim_regex.split(row.trim()).collect::<Vec<&str>>(); 
      if matches.len() == 0 {
        continue;
      }

      for i in 0..matches.len() {
        if problems.len() <= i {
          problems.push(MathProblem::new());
        }

        if matches[i] == "" {
          continue;
        } if matches[i] == "+" || matches[i] == "-" || matches[i] == "*" || matches[i] == "/" {
          problems[i].operator = match matches[i] {
            "+" => Op::ADD,
            "-" => Op::SUBTRACT,
            "*" => Op::MULTIPLY,
            "/" => Op::DIVIDE,
            _ => panic!("invalid operand")
          }
        } else {
          problems[i].operands.push(matches[i].parse::<i64>().expect("invalid operand"));
        }
      }
    }

    return problems;
  }

  fn parse_part2(&self, input: &str) -> Vec<MathProblem> {
    let mut results: Vec<MathProblem> = vec![];

    let grid: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect();

    let mut current_problem = MathProblem::new();

    for x in 0..grid[0].len() {
      let mut cur_num = 0;
      let mut empty = true;

      for y in 0..grid.len() {
        if grid[y][x] == ' ' {
          continue;
        } else if grid[y][x].is_numeric() {
          let digit = grid[y][x].to_digit(10).unwrap() as i64;
          cur_num = cur_num * 10 + digit;
          empty = false;
        } else {
          current_problem.operator = match grid[y][x] {
            '*' => Op::MULTIPLY,
            '+' => Op::ADD,
            _ => panic!("test")
          };
          empty = false;
        }
      }

      if empty {
        results.push(current_problem);
        current_problem = MathProblem::new();
      } else if cur_num != 0 {
        current_problem.operands.push(cur_num);
      }
    }

    if current_problem.operator != Op::INVALID {
      results.push(current_problem);
      current_problem = MathProblem::new();
    }

    return results;
  }

  fn sum_problems(&self, problems: Vec<MathProblem>) -> i64 {
    let mut sum = 0;
    for problem in problems {
      sum += problem.evaluate();
    }
    return sum;
  }
}

impl Solution for Day6Solution {
  fn part1(&self, input: &str) -> i64 {
    let problems = self.parse_part1(input);
    return self.sum_problems(problems);
  }

  fn part2(&self, input: &str) -> i64 {
    let problems = self.parse_part2(input);
    return self.sum_problems(problems);
  }
}