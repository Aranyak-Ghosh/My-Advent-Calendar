use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    vec,
};

use commons::{Answer, Solution};

struct DayFour {
    input_file: File,
}

const BASE_EXPONENT: u128 = 2;

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    line.split_once(":")
        .and_then(|(_, numbers)| numbers.split_once("|"))
        .and_then(|(winning_numbers, card_numbers)| {
            let parsed_winning_numbers = winning_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let parsed_card_numbers = card_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            Some((parsed_winning_numbers, parsed_card_numbers))
        })
        .unwrap()
}

impl Solution for DayFour {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0, |acc, line| {
            let trimmed_line = line.map(|l| l.trim().to_string()).unwrap();

            if !trimmed_line.is_empty() {
                let (winning_numbers, card_numbers) = parse_line(&trimmed_line);

                let intersection_count = winning_numbers.intersection(&card_numbers).count();

                if intersection_count > 0 {
                    return acc + BASE_EXPONENT.pow(intersection_count as u32 - 1);
                }
            }
            acc
        });

        Answer::new(accumulator)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);
        let mut card_number: u32 = 0;

        let lines = reader
            .lines()
            .filter_map(|l| l.map(|line| line.trim().to_string()).ok())
            .collect::<Vec<String>>();

        let mut card_process_count: Vec<u32> = vec![0; lines.len() + 1];

        lines.iter().for_each(|line| {
            let trimmed_line = line;
            if !trimmed_line.is_empty() {
                card_number += 1;
                card_process_count[card_number as usize] += 1;
                let (winning_numbers, card_numbers) = parse_line(&trimmed_line);

                let mut intersection_count = winning_numbers.intersection(&card_numbers).count();
                eprintln!("Card number {}", card_number);
                eprintln!("Intersection count {}", intersection_count);
                while intersection_count > 0 {
                    if card_number as usize + intersection_count < lines.len() {
                        card_process_count[(card_number as usize + intersection_count)] +=
                            card_process_count[card_number as usize];
                        intersection_count -= 1;
                    } else {
                        break;
                    }
                }
            }
        });

        Answer::new(card_process_count.iter().sum::<u32>() as u128)
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
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayFour = DayFour { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(13));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(30));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayFour = DayFour { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
