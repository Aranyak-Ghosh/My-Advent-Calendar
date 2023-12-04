use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use commons::{Answer, Solution};

struct DayThree {
    input_file: File,
}

fn get_num_string(position: usize, chars: &Vec<char>) -> String {
    let mut start = position;
    let mut end = position;

    while start > 0 && chars[start - 1].is_numeric() {
        start -= 1;
    }

    while end < (chars.len() - 1) && chars[end + 1].is_numeric() {
        end += 1;
    }

    chars[start..end + 1].iter().collect()
}

fn check_surrounding_cells(
    pos_row: usize,
    pos_col: usize,
    vec: Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    // We care about pos_col - 1, pos_col, pos_col + 1 and pos_row - 1, pos_row, pos_row + 1
    // One part can have multiple part numbers (one symbol can have multiple numbers around it)
    let row_start = if pos_row == 0 { pos_row } else { pos_row - 1 };
    let row_end = if pos_row == vec.len() {
        pos_row
    } else {
        pos_row + 1
    };

    let col_start = if pos_col == 0 { pos_col } else { pos_col - 1 };
    let col_end = if pos_col == vec[pos_row].len() {
        pos_col
    } else {
        pos_col + 1
    };

    let mut res = Vec::new();

    for i in row_start..row_end + 1 {
        for j in col_start..col_end + 1 {
            if vec[i][j].is_numeric() {
                if j == col_end || !vec[i][j + 1].is_numeric() {
                    res.push((i, j));
                    
                }
            }
        }
    }
    res
}

impl Solution for DayThree {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut buf = String::new();

        self.input_file.read_to_string(&mut buf).unwrap();

        let lines = buf
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut accumulator: u32 = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.iter().enumerate() {
                if !ch.is_numeric() && !ch.is_control() && *ch != '.' {
                    let part_val: u32 = check_surrounding_cells(i, j, lines.clone())
                        .iter()
                        .map(|(row, col)| {
                            let st = get_num_string(*col, &lines[*row]);

                            st.parse::<u32>().unwrap()
                        })
                        .sum();

                    accumulator += part_val;
                }
            }
        }

        Answer::new(accumulator as u128)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut buf = String::new();

        self.input_file.read_to_string(&mut buf).unwrap();

        let lines = buf
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut accumulator: u32 = 0_u32;

        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.iter().enumerate() {
                if *ch == '*' {
                    let part_vals = check_surrounding_cells(i, j, lines.clone());

                    let part_val: u32 = if part_vals.len() == 2 {
                        part_vals
                            .iter()
                            .map(|(row, col)| {
                                get_num_string(*col, &lines[*row]).parse::<u32>().unwrap()
                            })
                            .product()
                    } else {
                        0
                    };

                    accumulator += part_val;
                }
            }
        }

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
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayThree = DayThree { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(4361));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(467835));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayThree = DayThree { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
