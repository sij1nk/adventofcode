use anyhow::anyhow;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn to_char(self) -> char {
        match self {
            HandType::HighCard => 't',
            HandType::OnePair => 'u',
            HandType::TwoPair => 'v',
            HandType::ThreeOfAKind => 'w',
            HandType::FullHouse => 'x',
            HandType::FourOfAKind => 'y',
            HandType::FiveOfAKind => 'z',
        }
    }
}

const LEXICAL_TRANSFORM_TABLE: &[(char, char)] = &[
    ('A', 'Z'),
    ('K', 'Y'),
    ('Q', 'X'),
    ('J', 'W'),
    ('T', 'V'),
    ('9', 'U'),
    ('8', 'T'),
    ('7', 'S'),
    ('6', 'R'),
    ('5', 'Q'),
    ('4', 'P'),
    ('3', 'O'),
    ('2', 'N'),
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hand {
    value: String,
}

fn get_unique_cards_count(s: &str) -> u32 {
    let mut count = 0;
    let mut array = ['0'; 5];

    for c in s.chars() {
        if !array.contains(&c) {
            array[count] = c;
            count += 1;
        }
    }

    count as u32
}

fn get_highest_card_occurrence(s: &str) -> u32 {
    let mut highest = 0;
    let mut highest_char = None;
    let mut rem = 5;

    for char in s.chars() {
        if highest_char.is_some_and(|c| c == char) {
            continue;
        }

        let count = s.chars().filter(|&c| c == char).count();

        rem -= count;
        if highest < count {
            highest = count;
            highest_char = Some(char);
        }

        if highest >= rem {
            break;
        }
    }

    highest as u32
}

fn determine_hand_type(s: &str) -> HandType {
    let unique_count = get_unique_cards_count(s);

    match unique_count {
        1 => HandType::FiveOfAKind,
        2 => match get_highest_card_occurrence(s) {
            3 => HandType::FullHouse,
            4 => HandType::FourOfAKind,
            _ => panic!("Incorrect highest card occurrence number for unique count 2"),
        },
        3 => match get_highest_card_occurrence(s) {
            2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            _ => panic!("Incorrect highest card occurrence number for unique count 3"),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Number of unique cards higher than 5"),
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = String::with_capacity(6);

        let hand_type = determine_hand_type(s);
        value.push(hand_type.to_char());

        for c in s.chars() {
            let transform = LEXICAL_TRANSFORM_TABLE
                .iter()
                .find(|&item| item.0 == c)
                .expect("Char could not be translated");
            value.push(transform.1);
        }

        Ok(Hand { value })
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut hands: BTreeMap<Hand, u64> = BTreeMap::new();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            continue;
        }

        let (hand_str, winnings_str) = line
            .split_once(' ')
            .ok_or(anyhow!("Could not split line by ' '"))?;
        let hand = Hand::from_str(hand_str)?;
        let winnings = winnings_str.parse::<u64>()?;

        hands.insert(hand, winnings);
    }

    Ok(hands
        .values()
        .enumerate()
        .fold(0, |acc, (i, w)| acc + (i + 1) as u64 * w))
}

pub fn part2<'a, I, S>(_lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
