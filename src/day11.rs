use std::collections::HashMap;

use crate::solution::Solution;

struct Device {
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
      device_map.insert(name.clone(), Device { outputs: output_names });
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

  fn part2_helper(&self, devices: &HashMap<String, Device>, current: String, target: &str, cache: &mut HashMap<String, i64>) -> i64 {
    if current == target {
      return 1;
    }

    if current == "out" {
      return 0;
    }

    if cache.contains_key(&current) {
      return *cache.get(&current).unwrap();
    }

    let device = devices.get(&current).expect(format!("undefined device {}", current).as_str());

    let mut  total = 0;
    for output in device.outputs.iter() {
      total += self.part2_helper(devices, output.clone(), target, cache);
    }

    cache.insert(current, total);
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
    
    let srv_to_fft = self.part2_helper(&devices, "svr".to_owned(), "fft", &mut HashMap::new());
    let fft_to_dac = self.part2_helper(&devices, "fft".to_owned(), "dac", &mut HashMap::new());
    let dac_to_out = self.part2_helper(&devices, "dac".to_owned(), "out", &mut HashMap::new());

    let srv_to_dac = self.part2_helper(&devices, "svr".to_owned(), "dac", &mut HashMap::new());
    let dac_to_fft = self.part2_helper(&devices, "dac".to_owned(), "fft", &mut HashMap::new());
    let fft_to_out = self.part2_helper(&devices, "fft".to_owned(), "out", &mut HashMap::new());

    return srv_to_fft * fft_to_dac * dac_to_out + srv_to_dac * dac_to_fft * fft_to_out;
  }
}