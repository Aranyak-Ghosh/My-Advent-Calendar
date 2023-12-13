use std::{
    fs::File,
    io::Read,
};

use commons::{Answer, Solution};

struct DayFourteen {
    input_file: File,
}

impl Solution for DayFourteen {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut accumulator: u32 = 0_u32;

        Answer::new(accumulator as u128)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut accumulator: u32 = 0_u32;

        Answer::new(accumulator as u128)
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
        
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayFourteen = DayFourteen { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(0));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(0));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayFourteen = DayFourteen { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
