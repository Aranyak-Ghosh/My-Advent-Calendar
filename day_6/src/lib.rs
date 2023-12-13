use std::{fs::File, io::Read};

use commons::{Answer, Solution};

const SPEED_INCREMENT: u32 = 1;

struct DaySix {
    input_file: File,
}

impl Solution for DaySix {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut buf = String::new();
        self.input_file.read_to_string(&mut buf).unwrap();
        let mut tmp = buf.split("\n").filter(|x| !x.is_empty());
        let time = tmp
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());
        let distance = tmp
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let accumulator: u32 = time
            .zip(distance)
            .map(|(t, d)| {
                let mut wind_up_time = 0;
                let mut possible_counts = 0_u32;
                while wind_up_time < t {
                    let distance_travelled = (t - wind_up_time) * wind_up_time * SPEED_INCREMENT;
                    if distance_travelled > d {
                        possible_counts += 1;
                    }
                    wind_up_time += 1;
                }
                possible_counts
            })
            .product();

        Answer::new(accumulator as u128)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut buf = String::new();
        self.input_file.read_to_string(&mut buf).unwrap();
        let mut tmp = buf.split("\n").filter(|x| !x.is_empty());
        let time = tmp
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        let distance = tmp
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<u128>()
            .unwrap();

        let mut wind_up_time = 0;
        let mut possible_counts = 0_u128;
        while wind_up_time < time {
            let distance_travelled = (time - wind_up_time) * wind_up_time * (SPEED_INCREMENT as u128);
            if distance_travelled > distance {
                possible_counts += 1;
            }
            wind_up_time += 1;
        }

        Answer::new(possible_counts)
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
        Time:      7  15   30
        Distance:  9  40  200
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DaySix = DaySix { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(288));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(71503));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DaySix = DaySix { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
