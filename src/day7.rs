use std::collections::{HashMap, HashSet, VecDeque};

use crate::solution::Solution;

struct TachyonManifold {
  start_x: usize,
  start_y: usize,
  splitters: Vec<(usize, usize)>,
  splitters_by_col: HashMap<usize, Vec<usize>>
}

impl TachyonManifold {
  fn parse(input: &str) -> TachyonManifold {
    let mut manifold = TachyonManifold{
      start_x: 0,
      start_y: 0,
      splitters: vec![],
      splitters_by_col: HashMap::new()
    };

    let mut x: usize;
    let mut y: usize = 0;

    for row in input.split("\n") {
      x = 0;
      for tile in row.chars() {
        match tile {
          'S' => {
            manifold.start_x = x;
            manifold.start_y = y;
          },
          '^' => {
            manifold.splitters.push((x, y));

            if !manifold.splitters_by_col.contains_key(&x) {
              manifold.splitters_by_col.insert(x, vec![]);
            }
            manifold.splitters_by_col.get_mut(&x).unwrap().push(y); 
          },
          _ => {}
        }

        x += 1;
      }

      y+=1;
    }

    manifold
  }
}

pub struct Day7Solution {}

impl Day7Solution {
  fn part2_helper(&self, m: &TachyonManifold, sx: usize, sy: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
    if cache.contains_key(&(sx, sy)) {
      return *cache.get(&(sx, sy)).unwrap();
    }
    
    let column = match m.splitters_by_col.get(&sx) {
      Some(x) => x,
      None => return 1
    };

    let mut splitter_y: Option<usize> = Option::None;
    for i in column {
      if *i > sy {
        splitter_y = Some(*i);
        break;
      }
    }

    if splitter_y.is_none() {
      return 1;
    }

    let ret = self.part2_helper(m, sx-1, splitter_y.unwrap(), cache) + self.part2_helper(m, sx+1, splitter_y.unwrap(), cache);
    cache.insert((sx, sy), ret);
    return ret;
  
  }
}

impl Solution for Day7Solution {
  fn part1(&self, input: &str) -> i64 {
    let manifold = TachyonManifold::parse(input);
    let mut hit: HashSet<(usize, usize)> = HashSet::new();
    let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
    beams.push_back((manifold.start_x, manifold.start_y));

    while !beams.is_empty() {
      let top = beams.pop_front().unwrap();
      let column = match manifold.splitters_by_col.get(&top.0) {
        Some(x) => x,
        None => continue
      };
      
      for i in column {
        if *i > top.1 {
          if !hit.contains(&(top.0, *i)) {
            hit.insert((top.0, *i));
            beams.push_back((top.0 - 1, *i));
            beams.push_back((top.0 + 1, *i));
          }
          break;
        }
      }
    }

    return hit.len() as i64;
  }

  fn part2(&self, input: &str) -> i64 {
    let manifold = TachyonManifold::parse(input);
    let mut cache = HashMap::new();
    return self.part2_helper(&manifold, manifold.start_x, manifold.start_y, &mut cache);
  }
}