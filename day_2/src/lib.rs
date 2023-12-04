use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use commons::{Answer, Solution};

// Very unnecessary but doing it anyway

const RED: &str = "red";
const BLUE: &str = "blue";
const GREEN: &str = "green";

enum Colours {
    Red,
    Blue,
    Green,
    Unknown,
}

impl Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Colours::Red => write!(f, "{}", RED),
            Colours::Blue => write!(f, "{}", BLUE),
            Colours::Green => write!(f, "{}", GREEN),
            Colours::Unknown => write!(f, "Unknown colour found. Sanitize input"),
        }
    }
}

impl From<&str> for Colours {
    fn from(val: &str) -> Self {
        match val.to_lowercase().as_ref() {
            RED => Colours::Red,
            BLUE => Colours::Blue,
            GREEN => Colours::Green,
            _ => Colours::Unknown,
        }
    }
}

struct DayTwo {
    input_file: File,
    cube_max_counts: CubeCount,
}

#[derive(Default, Debug)]
struct CubeCount {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeCount {
    pub fn max_count(&mut self, color: Colours, current_val: u32) {
        match color {
            Colours::Red => {
                if self.red <= current_val {
                    self.red = current_val;
                }
            }
            Colours::Blue => {
                if self.blue <= current_val {
                    self.blue = current_val;
                }
            }
            Colours::Green => {
                if self.green <= current_val {
                    self.green = current_val;
                }
            }
            Colours::Unknown => {}
        }
    }
}

// We could go with the assumption that the line number indicates game number
// and ignore the game number from actual line but going to explicitly parse
// game number from line in this function

fn parse_line(line: &str) -> (u32, CubeCount) {
    let mut cube_counts = CubeCount::default();

    let (game_config, cube_config) = line.split_once(":").unwrap();

    // parse game number
    let game_number = game_config
        .split_whitespace()
        .last()
        .and_then(|x| Some(x.parse::<u32>()))
        .unwrap()
        .unwrap();

    // parse cube configurations

    for draw in cube_config.split(";") {
        draw.split(",").for_each(|x| {
            let mut config = x.trim().split_whitespace();

            let count = config
                .next()
                .and_then(|x| Some(x.parse::<u32>()))
                .unwrap()
                .unwrap();
            let color = Colours::from(config.last().unwrap());

            cube_counts.max_count(color, count);
        })
    }

    (game_number, cube_counts)
}

impl DayTwo {
    fn is_valid_game(&self, input: CubeCount) -> bool {
        input.red <= self.cube_max_counts.red
            && input.blue <= self.cube_max_counts.blue
            && input.green <= self.cube_max_counts.green
    }
}
impl Solution for DayTwo {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
            let trim_line = line.map(|s| s.trim().to_string()).unwrap();
            if !trim_line.is_empty() {
                let (game_id, cube_counts) = parse_line(&trim_line);

                if self.is_valid_game(cube_counts) {
                    return acc + game_id as u128;
                }
            }
            acc
        });
        Answer::new(accumulator)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let reader = BufReader::new(&self.input_file);

        let accumulator: u128 = reader.lines().fold(0_u128, |acc, line| {
            let trim_line = line.map(|s| s.trim().to_string()).unwrap();
            if !trim_line.is_empty() {
                let (_, cube_counts) = parse_line(&trim_line);

                return acc + (cube_counts.red * cube_counts.blue * cube_counts.green) as u128;
            }
            acc
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
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DayTwo = DayTwo {
            input_file: file,
            cube_max_counts: CubeCount {
                red: 12,
                green: 13,
                blue: 14,
            },
        };

        assert_eq!(test_case.part_a(), Answer::new(8));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(2286));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DayTwo = DayTwo {
            input_file: input,
            cube_max_counts: CubeCount {
                red: 12,
                green: 13,
                blue: 14,
            },
        };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
