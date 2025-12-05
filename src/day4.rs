use crate::solution::Solution;

#[derive(Clone, Copy, PartialEq)]
enum TileType {
  EMPTY,
  ROLL
}

struct Grid {
  data: Vec<Vec<TileType>>
}

impl Grid {
  fn parse(file: &str) -> Grid {
    let mut data: Vec<Vec<TileType>> = vec![];

    for line in file.split("\n") {
      let mut row: Vec<TileType> = vec![];
      
      for c in line.chars() {
        let tile_type = match c {
          '@' => TileType::ROLL,
          '.' => TileType::EMPTY,
          _ => panic!("invalid tile character {}", c)
        };

        row.push(tile_type);
      }

      data.push(row);
    }

    return Grid { data: data };
  }

  fn get(&self, x: usize, y: usize) -> TileType {
    return match self.data.get(y) {
      Some(row) => match row.get(x) {
        Some(x) => *x,
        None => TileType::EMPTY
      },
      None => TileType::EMPTY
    }
  }

  fn height(&self) -> usize {
    return self.data.len();
  }

  fn width(&self) -> usize {
    if self.height() == 0 {
      return 0;
    }

    return self.data[0].len();
  }
}

pub struct Day4Solution {}

impl Day4Solution {
  
}

impl Solution for Day4Solution {
  fn part1(&self, inp: &str) -> i64 {
    let grid = Grid::parse(inp);
    let mut available = 0;

    for y in 0..grid.height() {
      for x in 0..grid.width() {
        if grid.get(x, y) == TileType::ROLL {
          let mut adjacent = 0;
          for i in -1_i64..2 {
            for j in -1_i64..2 {
              let tile_type = grid.get((x as i64 + i) as usize,(y as i64 + j) as usize);
              if !(i == 0 && j == 0) && tile_type == TileType::ROLL {
                adjacent += 1;
              }
            }
          }

          if adjacent < 4 {
            available += 1;
          }
        }
      }
    }

    return available;
  }

  fn part2(&self, inp: &str) -> i64 {
    return 0;
  }
}