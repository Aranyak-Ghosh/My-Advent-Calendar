use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use commons::{Answer, Solution};

struct DayEight {
    input_file: File,
}

#[derive(Clone)]
struct NextLocation {
    l: String,
    r: String,
}

const FINAL_DESTINATION: &str = "ZZZ";

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl Solution for DayEight {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut num_steps: u32 = 0_u32;

        let mut lines = BufReader::new(&self.input_file).lines().filter_map(|line| {
            line.ok()
                .filter(|x| !x.trim().trim_matches('\0').is_empty())
        });

        let instruction_str = lines.next().unwrap();
        let mut instructions = instruction_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .cycle(); // Loop instructions infinitely

        let mut map: HashMap<String, NextLocation> = HashMap::new();

        while let Some(line) = lines.next() {
            let (source, destinations) = line.split_once("=").unwrap();
            let source = source.trim();
            let (left, right) = destinations
                .split_once(",")
                .map(|(s, d)| (s.trim().trim_matches('('), d.trim().trim_matches(')')))
                .unwrap();

            map.insert(
                source.to_string(),
                NextLocation {
                    l: left.to_string(),
                    r: right.to_string(),
                },
            );
        }

        let mut current_position = "AAA".to_string();

        while current_position != FINAL_DESTINATION.to_string() {
            let next_position = match instructions.next().unwrap() {
                'L' => map.get(&current_position).unwrap().l.clone(),
                'R' => map.get(&current_position).unwrap().r.clone(),
                _ => panic!("Invalid instruction"),
            };
            num_steps += 1;
            current_position = next_position;
        }

        Answer::new(num_steps as u128)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut num_steps = 1_u64;

        let mut lines = BufReader::new(&self.input_file).lines().filter_map(|line| {
            line.ok()
                .filter(|x| !x.trim().trim_matches('\0').is_empty())
        });

        let instruction_str = lines.next().unwrap();
        let mut instructions = instruction_str.chars().filter(|c| !c.is_whitespace());

        let mut map: HashMap<String, NextLocation> = HashMap::new();

        while let Some(line) = lines.next() {
            let (source, destinations) = line.split_once("=").unwrap();
            let source = source.trim();
            let (left, right) = destinations
                .split_once(",")
                .map(|(s, d)| (s.trim().trim_matches('('), d.trim().trim_matches(')')))
                .unwrap();

            map.insert(
                source.to_string(),
                NextLocation {
                    l: left.to_string(),
                    r: right.to_string(),
                },
            );
        }

        let current_positions = map.keys().filter_map(|k| {
            if k.ends_with("A") {
                Some(k.to_string())
            } else {
                None
            }
        });

        let ind_num_steps = current_positions.map(|mut c| {
            let mut local_num_steps = 0_u64;
            let mut local_instructions = instructions.clone().cycle();

            while !c.ends_with("Z") {
                let next_position = match local_instructions.next().unwrap() {
                    'L' => map.get(&c).unwrap().l.clone(),
                    'R' => map.get(&c).unwrap().r.clone(),
                    _ => panic!("Invalid instruction"),
                };
                local_num_steps += 1;
                c = next_position;
            }

            local_num_steps
        }).collect::<Vec<u64>>();

        for i in 0..ind_num_steps.len() {
            num_steps = lcm(num_steps, ind_num_steps[i])
        }

        Answer::new(num_steps as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, Write};
    use tempfile::tempfile;

    #[test]
    fn test() {
        let input_a = b"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayEight = DayEight { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(6));

        let input_b = b"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        ";

        test_case.input_file.set_len(0).unwrap();

        test_case.input_file.write_all(input_b).unwrap();

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(6));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayEight = DayEight { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
