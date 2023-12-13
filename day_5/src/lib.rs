use std::{fs::File, io::Read};

use commons::{Answer, Solution};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

struct DayFive {
    input_file: File,
}

struct RangeEntry {
    src_start: u128,
    dest_start: u128,
    range_len: u128,
}

impl From<&str> for RangeEntry {
    fn from(input: &str) -> Self {
        let parsed_input = input
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        assert_eq!(parsed_input.len(), 3);

        RangeEntry {
            dest_start: parsed_input[0],
            src_start: parsed_input[1],
            range_len: parsed_input[2],
        }
    }
}

impl Solution for DayFive {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut file_content = String::new();
        self.input_file.read_to_string(&mut file_content).unwrap();

        let mut file_sections = file_content.split("\n\n").map(|x| x.trim());

        let mut source = file_sections
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        let mut destination = source.clone();

        while let Some(section) = file_sections.next() {
            source = destination.clone();

            let range_map = section
                .split("\n")
                .filter(|x| !x.contains(":"))
                .map(|x| RangeEntry::from(x))
                .collect::<Vec<_>>();

            destination = source
                .clone()
                .iter()
                .map(|start| {
                    let mut dest = *start;
                    for range in &range_map {
                        let offset = *start as i128 - range.src_start as i128;
                        if offset >= 0 && offset as u128 <= range.range_len {
                            dest = range.dest_start + offset as u128;
                            break;
                        }
                    }
                    dest
                })
                .collect::<Vec<u128>>();
        }

        Answer::new(*destination.iter().min().unwrap())
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut file_content = String::new();
        self.input_file.read_to_string(&mut file_content).unwrap();

        let mut file_sections = file_content.split("\n\n").map(|x| x.trim());

        let mut source = file_sections
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<u128>>()
            .par_chunks_exact(2_usize)
            .flat_map(|x| {
                let start = x[0];
                let end = x[0] + x[1];
                start..end
            })
            .collect::<Vec<_>>();

        let mut destination = Vec::new();

        while let Some(section) = file_sections.next() {
            if destination.len() != 0 {
                source = destination.clone();
            }

            let range_map = section
                .split("\n")
                .filter(|x| !x.contains(":"))
                .map(|x| RangeEntry::from(x))
                .collect::<Vec<_>>();

            destination = source
                .par_iter()
                .map(|start| {
                    let mut dest = *start;
                    for range in &range_map {
                        let offset = *start as i128 - range.src_start as i128;
                        if offset >= 0 && offset as u128 <= range.range_len {
                            dest = range.dest_start + offset as u128;
                            break;
                        }
                    }
                    dest
                })
                .collect::<Vec<u128>>();
        }

        Answer::new(*destination.iter().min().unwrap())
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
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayFive = DayFive { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(35_u128));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(46_u128));
    }

    #[test]
    fn execute() {
        let input = File::open("/Users/aranyakg/workplace/aoc_2023/src/day_5/input.txt").unwrap();

        let mut test_case: DayFive = DayFive { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
