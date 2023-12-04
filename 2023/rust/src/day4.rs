use std::collections::BTreeMap;

use anyhow::anyhow;

type CardId = u32;
type CardHits = u32;
type CardAmount = u32;

fn parse_card(line: &str) -> anyhow::Result<CardHits> {
    let (_, numbers) = line
        .split_once(':')
        .ok_or(anyhow!("Invalid input line - could not split by ':'"))?;
    let (winning_numbers_str, our_numbers_str) = numbers
        .split_once('|')
        .ok_or(anyhow!("Invalid input line - could not split by '|'"))?;
    let winning_numbers: Vec<u32> = winning_numbers_str
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let our_numbers: Vec<u32> = our_numbers_str
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let hits = winning_numbers
        .iter()
        .filter(|&n| our_numbers.contains(n))
        .count() as CardHits;

    Ok(hits)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut points: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            continue;
        }

        let hits = parse_card(line)?;

        if hits > 0 {
            points += 2u32.pow((hits - 1) as u32);
        }
    }

    Ok(points)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut processed_cards: u32 = 0;
    let mut cards: BTreeMap<CardId, (CardHits, CardAmount)> = BTreeMap::new();

    for (id, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        let id = (id + 1) as CardId;
        if line.is_empty() {
            continue;
        }

        let hits = parse_card(line)?;
        cards.insert(id, (hits, 1));
    }

    let mut id = 1;
    loop {
        let Some((hits, amount)) = cards.get(&id).copied() else {
            break;
        };

        for n in id + 1..id + 1 + hits {
            if let Some((_, other_amount)) = cards.get_mut(&n) {
                *other_amount += amount;
            }
        }

        processed_cards += amount;
        id += 1;
    }

    Ok(processed_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 30);
    }
}
