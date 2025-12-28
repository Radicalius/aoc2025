use crate::solution::Solution;

#[derive(Debug)]
struct Shape {
  grid: [[bool; 3]; 3]
}

impl Shape {
  fn area(&self) -> i64 {
    let mut a = 0;
    for i in self.grid {
      for j in i {
        if j {
          a += 1;
        }
      }
    }

    return a;
  }
}

#[derive(Debug)]
struct Problem {
  width: u32,
  height: u32,
  shapes: Vec<u32>
}

impl Problem {
  fn parse(input: &str) -> Problem {
    let parts = input.split(": ").collect::<Vec<&str>>();
    assert!(parts.len() == 2);
    let dims = parts[0].split("x").collect::<Vec<&str>>();
    let w = dims[0].parse::<u32>().expect(format!("invalid width in problem: {}", input).as_str());
    let h = dims[1].parse::<u32>().expect(format!("invalid width in problem: {}", input).as_str());
    let packages = parts[1]
      .split(" ")
      .map(|x| x.parse::<u32>()
        .expect(format!("invalid: {}", input).as_str()
      )).collect::<Vec<u32>>();

    return Problem { width: w, height: h, shapes: packages }
  }
}

pub struct Day12Solution {}

impl Day12Solution {
  fn parse(input: &str) -> (Vec<Shape>, Vec<Problem>) {
    let mut shapes = Vec::new();
    let mut problems = Vec::new();
    
    let stripped_inp = input.replace("\r", "");
    let lines = stripped_inp.split("\n").collect::<Vec<&str>>();
    let mut line = 0;
    while line < lines.len() {
      let parts = lines[line].split(":").collect::<Vec<&str>>();
      if parts.len() == 1 {
        line += 1;
        continue;
      }

      if parts[1] == "" {
        let mut s = Shape{ grid: [[false; 3]; 3] };
        for o in 0..3 {
          line+=1;
          for (h,c) in lines[line].chars().enumerate() {
            s.grid[o][h] = c == '#'; 
          }
        }

        shapes.push(s);
      } else {
        problems.push(Problem::parse(lines[line]));
      }

      line += 1;
    }

    return (shapes, problems);
  }
}

impl Solution for Day12Solution {
  fn part1(&self, input: &str) -> i64 {
    let (shapes, problems) = Day12Solution::parse(input);
    let mut possible = 0;
    for problem in problems {
      let total_area: i64 = problem.shapes.iter().enumerate().map(|(i,c)| shapes[i].area() * *c as i64).sum();
      let total_shapes: u32 = problem.shapes.iter().sum();
      if problem.width * problem.height < total_area as u32 {
        // Impossible
      } else if problem.width * problem.height >= 9 * total_shapes {
        possible += 1;
      } else {
        println!("{:?}", problem);
        assert!(false, "Unimplemented");
      }
    }
    
    return possible;
  }

  fn part2(&self, input: &str) -> i64 {
    return 0;
  }
}