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

impl DayOne {
    pub fn reset(&mut self) {
        self.input_file.set_len(0).unwrap();
    }
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
        todo!()
        // let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
        //     let line = line.unwrap();

        //     let tens: u32 = num_iter.next().unwrap_or_default();
        //     let ones: u32 = num_iter.last().unwrap_or(tens);

        //     acc + (tens * 10 + ones) as u128
        // });
        // Answer::from(accumulator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, Read, Seek};
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

        let mut test_case: DayOne = DayOne {
            input_file: file,
        };

        assert_eq!(test_case.part_a(), Answer::new(142));


        let input_b = b"";
        
        test_case.reset();
        
        test_case.input_file.write_all(input_b).unwrap();

        assert_eq!(test_case.part_a(), Answer::new(0));

    }
}
