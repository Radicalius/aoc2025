use std::{fs, time::Instant};

use clap::Parser;

mod solution;
use solution::Solution;

mod day1;
use day1::Day1Solution;

mod day2;
use day2::Day2Solution;

mod day3;
use day3::Day3Solution;

mod day4;
use day4::Day4Solution;

mod day5;
use day5::Day5Solution;

mod day6;
use day6::Day6Solution;

mod day7;
use day7::Day7Solution;

mod day8;
use day8::Day8Solution;

mod day9;
use day9::Day9Solution;

mod day10;
use day10::Day10Solution;

/// Solutions to advent of code 2025
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Problem day number
  #[arg(short, long, default_value_t = 0)]
  day: usize,

  /// Problem part
  #[arg(short, long, default_value_t = 0)]
  part: usize,

  /// Problem input
  #[arg(short, long, default_value_t = 0)]
  input: usize
}

fn time<SolutionFunc>(f: SolutionFunc, input: &str) -> (i64, u128) where SolutionFunc: Fn(&str) -> i64 {
  let start = Instant::now();
  let result = f(input);
  let duration = start.elapsed();
  return (result, duration.as_micros());
}

fn main() {
  let args = Args::parse();

  let solutions: Vec<Box<dyn Solution>> = vec![
    Box::from(Day1Solution{}),
    Box::from(Day2Solution{}),
    Box::from(Day3Solution{}),
    Box::from(Day4Solution{}),
    Box::from(Day5Solution{}),
    Box::from(Day6Solution{}),
    Box::from(Day7Solution{}),
    Box::from(Day8Solution{}),
    Box::from(Day9Solution{}),
    Box::from(Day10Solution{})
  ];

  let solution: &Box<dyn Solution>  = match solutions.get(args.day - 1) {
    Some(x) => x,
    None => panic!("solution for day not yet implemented")
  };

  let sample_filename = format!("inputs/{}-sample.txt", args.day);
  let sample_input = fs::read_to_string(&sample_filename).expect(&format!("error opening input file {}", sample_filename));

  let full_filename = format!("inputs/{}-full.txt", args.day);
  let full_input = fs::read_to_string(&full_filename).expect(&format!("error opening input file {}", full_filename));

  println!("Problem {}", args.day);

  if args.part == 0 || args.part == 1 {
    println!("  Part 1");

    if args.input == 0 || args.input == 1 {
        let (part1_sample, part1_sample_time) = time (|x| solution.part1(x), &sample_input);
        println!("    Sample: {part1_sample} ({part1_sample_time} us)", );   
    }

    if args.input == 0 || args.input == 2 {
        let (part1_full, part1_full_time) = time (|x| solution.part1(x), &full_input);
        println!("    Full:   {} ({} us)", part1_full, part1_full_time);
    }
  }

  if args.part == 0 || args.part == 2 {
    println!("  Part 2");

    if args.input == 0 || args.input == 1 {
        let (part2_sample, part2_sample_time) = time (|x| solution.part2(x), &sample_input);
        println!("    Sample: {part2_sample} ({part2_sample_time} us)", );
    }

    if args.input == 0 || args.input == 2 {
        let (part2_full, part2_full_time) = time (|x| solution.part2(x), &full_input);
        println!("    Full:   {} ({} us)", part2_full, part2_full_time);
    }
  }
}
