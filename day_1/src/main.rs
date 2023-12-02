use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const BASE_10_RADIX: u32 = 10;

fn get_first_num(string: impl Iterator<Item = char>) -> Option<u32> {
    for c in string {
        if c.is_numeric() {
            return c.to_digit(BASE_10_RADIX);
        }
    }
    return None;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let accumulator: u128 = reader.lines().fold(0 as u128, |acc, line| {
        let line = line.unwrap();

        let tens: u32 = get_first_num(line.chars()).unwrap_or_default();
        let ones: u32 = get_first_num(line.chars().rev()).unwrap_or_default();

        acc + (tens * 10 + ones) as u128
    });

    println!("Total sum: {:?}", accumulator);
}
