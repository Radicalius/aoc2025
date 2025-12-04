use std::{env, fs};

mod solution;
use solution::Solution;

mod day2;
use day2::Day2Solution;

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
        Box::from(Day2Solution{}),
        Box::from(Day2Solution{})
    ];

    let filename = String::from("inputs/")+dayStr+".txt";
    let input = fs::read_to_string(filename).expect("error opening input file");
    
    let solution: &Box<dyn Solution>  = match solutions.get(day - 1) {
        Some(x) => x,
        None => panic!("solution for day not yet implemented")
    };

    println!("part 1: {}", solution.part1(&input));
    println!("part 2: {}", solution.part2(&input));
}
