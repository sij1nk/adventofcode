use anyhow::anyhow;
use std::collections::BTreeMap;

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

const LEXICAL_TRANSFORM_TABLE_PART2: &[(char, char)] = &[
    ('A', 'Z'),
    ('K', 'Y'),
    ('Q', 'X'),
    ('T', 'V'),
    ('9', 'U'),
    ('8', 'T'),
    ('7', 'S'),
    ('6', 'R'),
    ('5', 'Q'),
    ('4', 'P'),
    ('3', 'O'),
    ('2', 'N'),
    ('J', 'A'),
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hand {
    value: String,
}

fn get_unique_cards_count(s: &str, consider_jokers: bool) -> u32 {
    let mut count = 0;
    let mut array = ['0'; 5];

    for c in s.chars() {
        if consider_jokers && c == 'J' {
            continue;
        }

        if !array.contains(&c) {
            array[count] = c;
            count += 1;
        }
    }

    // JJJJJ corner case for part 2
    if count == 0 {
        1
    } else {
        count as u32
    }
}

fn get_highest_card_occurrence(s: &str, consider_jokers: bool) -> u32 {
    let jokers_count = if consider_jokers {
        s.chars().filter(|&c| c == 'J').count()
    } else {
        0
    };

    let mut highest = 0;
    let mut highest_char = None;
    let mut rem = 5 - jokers_count;

    for char in s.chars() {
        if consider_jokers && char == 'J' {
            continue;
        }

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

    (highest + jokers_count) as u32
}

fn determine_hand_type(s: &str, consider_jokers: bool) -> HandType {
    let unique_count = get_unique_cards_count(s, consider_jokers);

    match unique_count {
        1 => HandType::FiveOfAKind,
        2 => match get_highest_card_occurrence(s, consider_jokers) {
            3 => HandType::FullHouse,
            4 => HandType::FourOfAKind,
            _ => panic!("Incorrect highest card occurrence number for unique count 2"),
        },
        3 => match get_highest_card_occurrence(s, consider_jokers) {
            2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            _ => panic!("Incorrect highest card occurrence number for unique count 3"),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Number of unique cards higher than 5"),
    }
}

impl Hand {
    fn new(s: &str, consider_jokers: bool) -> Self {
        let mut value = String::with_capacity(6);

        let hand_type = determine_hand_type(s, consider_jokers);
        value.push(hand_type.to_char());

        let transform_table = if consider_jokers {
            LEXICAL_TRANSFORM_TABLE_PART2
        } else {
            LEXICAL_TRANSFORM_TABLE
        };

        for c in s.chars() {
            let transform = transform_table
                .iter()
                .find(|&item| item.0 == c)
                .expect("Char could not be translated");
            value.push(transform.1);
        }

        Hand { value }
    }
}

fn calculate_winnings<'a, I, S>(lines: I, consider_jokers: bool) -> anyhow::Result<u64>
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
        let hand = Hand::new(hand_str, consider_jokers);
        let winnings = winnings_str.parse::<u64>()?;

        hands.insert(hand, winnings);
    }

    Ok(hands
        .values()
        .enumerate()
        .fold(0, |acc, (i, w)| acc + (i + 1) as u64 * w))
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    calculate_winnings(lines, false)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    calculate_winnings(lines, true)
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

        assert_eq!(result, 5905);
    }
}
