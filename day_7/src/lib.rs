use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use commons::{Answer, Solution};

const HAND_SIZE: usize = 5;
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Card(char);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default, PartialOrd, Ord)]
struct CardValue(u32);
type Cards = Box<[CardValue]>;
type Bid = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

struct Hand {
    cards: Cards,
    bid: Bid,
    hand_type: HandType,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Part {
    A,
    B,
}

impl CardValue {
    fn from(c: Card, part: Part) -> Self {
        match c {
            Card('2') => CardValue(2),
            Card('3') => CardValue(3),
            Card('4') => CardValue(4),
            Card('5') => CardValue(5),
            Card('6') => CardValue(6),
            Card('7') => CardValue(7),
            Card('8') => CardValue(8),
            Card('9') => CardValue(9),
            Card('T') => CardValue(10),
            Card('J') => {
                if part == Part::A {
                    CardValue(11)
                } else {
                    CardValue(1)
                }
            }
            Card('Q') => CardValue(12),
            Card('K') => CardValue(13),
            Card('A') => CardValue(14),
            _ => panic!("Invalid Card!!!"),
        }
    }
}

impl Hand {
    fn from(s: String, part: Part) -> Self {
        if let Some((hand, bid)) = s.split_once(" ") {
            let mut unique_map: HashMap<Card, u32> = HashMap::new();
            let mut max_occurence = 0;
            let card_values: Cards = hand
                .chars()
                .map(|card| {
                    let card = Card(card);

                    let entry = unique_map.entry(card).or_default();

                    *entry += 1;

                    if *entry > max_occurence {
                        max_occurence = *entry;
                    }

                    CardValue::from(card, part)
                })
                .collect::<Vec<CardValue>>()
                .into_boxed_slice();

            assert_eq!(card_values.len(), HAND_SIZE);

            let unique_cards = unique_map.len();
            let j_count = *unique_map.entry(Card('J')).or_default();
            let contains_j = part == Part::B && j_count > 0;

            let hand_type = match max_occurence {
                5 => HandType::FiveOfAKind,
                4 => {
                    if contains_j {
                        // JJJJX or XXXXJ
                        HandType::FiveOfAKind
                    } else {
                        // XXXXY
                        HandType::FourOfAKind
                    }
                }
                3 => {
                    if unique_cards == 2 {
                        if contains_j {
                            // JJJXX or XXXJJ
                            HandType::FiveOfAKind
                        } else {
                            // XXXYY
                            HandType::FullHouse
                        }
                    } else {
                        if contains_j {
                            // JJJXY or XXXJY
                            HandType::FourOfAKind
                        } else {
                            // XXXYZ
                            HandType::ThreeOfAKind
                        }
                    }
                }
                2 => {
                    if unique_cards == 3 {
                        if contains_j {
                            // JJXXY or XXYYJ
                            if j_count == 2 {
                                HandType::FourOfAKind
                            } else {
                                HandType::FullHouse
                            }
                        } else {
                            // XXYYZ
                            HandType::TwoPair
                        }
                    } else {
                        if contains_j {
                            // JJXYZ or XXJYZ
                            HandType::ThreeOfAKind
                        } else {
                            // XXYZA
                            HandType::OnePair
                        }
                    }
                }
                1 => {
                    if contains_j {
                        // XJYZA
                        HandType::OnePair
                    } else {
                        HandType::HighCard
                    }
                }
                _ => panic!("Invalid hand"),
            };

            Self {
                cards: card_values,
                bid: bid.parse().unwrap(),
                hand_type: hand_type,
            }
        } else {
            panic!("Invalid input");
        }
    }
}

struct DaySeven {
    input_file: File,
}

impl Solution for DaySeven {
    type Item = u128;
    fn part_a(&mut self) -> Answer<u128> {
        let mut accumulator: u32 = 0_u32;

        let reader = BufReader::new(&self.input_file);

        let mut hands = reader
            .lines()
            .filter_map(|x| x.ok().filter(|y| !y.trim().is_empty()))
            .map(|line| Hand::from(line.trim().to_string(), Part::A))
            .collect::<Vec<_>>();

        hands.sort_by(|a, b| {
            let mut order = a.hand_type.cmp(&b.hand_type);

            if order == Ordering::Equal {
                let mut i = 0;

                while a.cards[i] == b.cards[i] {
                    i += 1;
                }

                if i == HAND_SIZE {
                    panic!("Identical hands");
                } else {
                    order = a.cards[i].cmp(&b.cards[i]);
                }
            }

            order
        });

        for (k, v) in hands.iter().enumerate() {
            accumulator += v.bid * (k as u32 + 1);
        }

        Answer::new(accumulator as u128)
    }

    fn part_b(&mut self) -> Answer<u128> {
        let mut accumulator: u32 = 0_u32;
        let reader = BufReader::new(&self.input_file);

        let mut hands = reader
            .lines()
            .filter_map(|x| x.ok().filter(|y| !y.trim().is_empty()))
            .map(|line| Hand::from(line.trim().to_string(), Part::B))
            .collect::<Vec<_>>();

        hands.sort_by(|a, b| {
            let mut order = a.hand_type.cmp(&b.hand_type);

            if order == Ordering::Equal {
                let mut i = 0;

                while a.cards[i] == b.cards[i] {
                    i += 1;
                }

                if i == HAND_SIZE {
                    panic!("Identical hands");
                } else {
                    order = a.cards[i].cmp(&b.cards[i]);
                }
            }

            order
        });

        for (k, v) in hands.iter().enumerate() {
            accumulator += v.bid * (k as u32 + 1);
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
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        ";

        let mut file = tempfile().unwrap();

        file.write_all(input_a).unwrap();

        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut test_case: DaySeven = DaySeven { input_file: file };

        assert_eq!(test_case.part_a(), Answer::new(6440));

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        assert_eq!(test_case.part_b(), Answer::new(5905));
    }

    #[test]
    fn execute() {
        let input = File::open("input.txt").unwrap();

        let mut test_case: DaySeven = DaySeven { input_file: input };

        eprintln!("Test case A: {}", test_case.part_a().to_string());

        test_case
            .input_file
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        eprintln!("Test case B: {}", test_case.part_b().to_string());
    }
}
