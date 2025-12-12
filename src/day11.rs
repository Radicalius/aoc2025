use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::solution::Solution;

struct Device {
  name: String,
  outputs: Vec<String>
}

pub struct Day11Solution {}

impl Day11Solution {
  fn parse(&self, input: &str) -> HashMap<String, Device> {
    let mut device_map = HashMap::new();

    for line in input.split("\n") {
      let parts = line.split(": ").collect::<Vec<&str>>();
      assert!(parts.len() > 1, "invalid input line: {}", line);

      let name = parts[0].to_owned();
      let output_names = parts[1].split(" ").map(|x| x.to_owned()).collect::<Vec<String>>();
      device_map.insert(name.clone(), Device { name: name.clone(), outputs: output_names });
    }

    return device_map;
  }

  fn part1_helper(&self, devices: &HashMap<String, Device>, current: String) -> i64 {
    if current == "out" {
      return 1;
    }

    let device = devices.get(&current).expect(format!("undefined device {}", current).as_str());

    let mut  total = 0;
    for output in device.outputs.iter() {
      total += self.part1_helper(devices, output.clone());
    }

    return total;
  }

  fn part2_helper(&self, devices: &HashMap<String, Device>, current: String, seen_dac: bool, seen_fft: bool, seen: &mut HashSet<String>) -> i64 {
    if current == "out" {
      println!("in");
      if seen_dac && seen_fft {
        return 1;
      } else {
        return 0;
      }
    }

    if seen.contains(&current) {
      return 0;
    }

    seen.insert(current.clone());

    let device = devices.get(&current).expect(format!("undefined device {}", current).as_str());

    let new_seen_dac = seen_dac || current == "dac";
    let new_seen_fft = seen_fft || current == "fft";

    let mut  total = 0;
    for output in device.outputs.iter() {
      total += self.part2_helper(devices, output.clone(), new_seen_dac, new_seen_fft, seen);
    }

    seen.remove(&current);

    return total;
  }
}

impl Solution for Day11Solution {
  fn part1(&self, input: &str) -> i64 {
    let devices = self.parse(input);
    return self.part1_helper(&devices, "you".to_owned());
  }

  fn part2(&self, input: &str) -> i64 {
    let devices = self.parse(input);
    return self.part2_helper(&devices, "svr".to_owned(), false, false, &mut HashSet::new());
  }
}