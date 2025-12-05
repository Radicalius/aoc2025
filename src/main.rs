use std::{env, fs};

mod solution;
use solution::Solution;

mod day1;
use day1::Day1Solution;

mod day2;
use day2::Day2Solution;

mod day3;
use day3::Day3Solution;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    let dayStr = match args.get(1) {
        Some(x) => x,
        None => panic!("Please provide the aoc day to run")
    };

    let day = match dayStr.parse::<usize>() {
        Ok(x) => x,
        Err(e) => panic!("error parsing aoc day argument")
    };

    assert!(day > 0 && day < 25, "day must be in [1, 25]");

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::from(Day1Solution{}),
        Box::from(Day2Solution{}),
        Box::from(Day3Solution{})
    ];

    let solution: &Box<dyn Solution>  = match solutions.get(day - 1) {
        Some(x) => x,
        None => panic!("solution for day not yet implemented")
    };

    let sample_filename = format!("inputs/{}-sample.txt", day);
    let sample_input = fs::read_to_string(&sample_filename).expect(&format!("error opening input file {}", sample_filename));

    println!("part 1 sample: {}", solution.part1(&sample_input));
    println!("part 2 sample: {}", solution.part2(&sample_input));

    let full_filename = format!("inputs/{}-full.txt", day);
    let full_input = fs::read_to_string(&full_filename).expect(&format!("error opening input file {}", full_filename));

    println!("part 1 full: {}", solution.part1(&full_input));
    println!("part 2 full: {}", solution.part2(&full_input));
}
