use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use commons::{Solution, Answer};

const BASE_10_RADIX: u32 = 10;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct DayOne {
    input_file: File,
}

impl Solution for DayOne {
    fn part_a(&self) -> commons::Answer {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
            let line = line.unwrap();
            let mut num_iter = line.chars().filter_map(|x| x.to_digit(BASE_10_RADIX));

            let tens: u32 = num_iter.next().unwrap_or_default();
            let ones: u32 = num_iter.last().unwrap_or(tens);

            acc + (tens * 10 + ones) as u128
        });
        Answer::from(accumulator)
    }

    fn part_b(&self) -> commons::Answer {
        todo!()
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let day_1 = DayOne { input_file: file };

    println!("Part A: {:?}", day_1.part_a());
}
