use std::fmt::Debug;

use regex::Regex;

use crate::solution::Solution;

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

struct MathProblem {
  operands: Vec<i64>,
  operator: Op
}

impl MathProblem {
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
  fn parse(&self, input: &str) -> Vec<MathProblem> {    
    let delim_regex = Regex::new(r"[^0-9^+^*^/^-]+").expect("invalid delim regex");
    
    let mut problems: Vec<MathProblem> = vec![];

    for row in input.split("\n") {
      let matches = delim_regex.split(row.trim()).collect::<Vec<&str>>(); 
      if matches.len() == 0 {
        continue;
      }

      for i in 0..matches.len() {
        if problems.len() <= i {
          problems.push(MathProblem{
            operands: Vec::new(),
            operator: Op::INVALID
          });
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
}

impl Solution for Day6Solution {
  fn part1(&self, input: &str) -> i64 {
    let problems = self.parse(input);
    let mut sum = 0;
    for problem in problems {
      sum += problem.evaluate();
    }
    return sum;
  }

  fn part2(&self, input: &str) -> i64 {
    return 0;
  }
}