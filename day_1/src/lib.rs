use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use commons::{Answer, Solution};

const BASE_10_RADIX: u32 = 10;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct DayOne {
    input_file: File,
}

impl Solution for DayOne {
    type Item = u128;
    fn part_a(&self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
            let trim_line = line.map(|s| s.trim().to_string()).unwrap();

            let mut num_iter = trim_line.chars().filter_map(|x| x.to_digit(BASE_10_RADIX));

            let tens: u32 = num_iter.next().unwrap_or_default();
            let ones: u32 = num_iter.last().unwrap_or(tens);

            acc + (tens * 10 + ones) as u128
        });
        Answer::new(accumulator)
    }

    fn part_b(&self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
            let line = line.map(|l| l.trim().to_string()).unwrap();
            let mut num_iter = Vec::<u32>::new();
            let enumerator = line.chars().enumerate();
            for (i, c) in enumerator {
                if c.is_ascii_digit() {
                    num_iter.push(c.to_digit(BASE_10_RADIX).unwrap());
                } else {
                    for (j, alpha_digit) in DIGITS.iter().enumerate() {
                        if line[i..].starts_with(alpha_digit) {
                            num_iter.push(j as u32 + 1);
                            // ideally no reason to check every character when substring match is found
                            // but skipping enumerator is not possible within the
                            // enumerator.skip(alpha_digit.len());
                        }
                    }
                }
            }

            let tens: u32 = num_iter.first().map(|x| x.to_owned()).unwrap_or_default();

            let ones: u32 = num_iter.last().map(|x| x.to_owned()).unwrap_or(tens);

            acc + (tens * 10 + ones) as u128
        });
        Answer::new(accumulator)
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
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayOne = DayOne { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(142));

        let input_b = b"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        test_case.input_file.set_len(0).unwrap();

        test_case.input_file.write_all(input_b).unwrap();

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(281));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayOne = DayOne { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
