use std::{cmp::Ordering, collections::{HashMap, BTreeMap}};

use itertools::Itertools;

pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut line_count = 0;
    let mut one_bits: BTreeMap<u32, u32> = BTreeMap::new();
    for line in lines.into_iter().map(|line| line.as_ref()) {
        line_count += 1;

        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                *one_bits.entry(i as u32).or_insert(0) += 1;
            }
        }
    }

    let mut gamma_binary_str = String::new();
    let mut epsilon_binary_str = String::new();

    for (_k, v) in one_bits.into_iter().sorted_by(|(k1, _), (k2, _)| {
        if k1 < k2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }) {
        if v > line_count / 2 {
            gamma_binary_str.push('1');
            epsilon_binary_str.push('0');
        } else {
            gamma_binary_str.push('0');
            epsilon_binary_str.push('1');
        }
    }

    let gamma = u32::from_str_radix(&gamma_binary_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_binary_str, 2).unwrap();

    Some(gamma * epsilon)
}

enum BitCriteria {
    LeastCommon,
    MostCommon,
}

fn part2_filter_by_bit_criteria<'a>(lines: &'a mut Vec<&str>, criteria: BitCriteria) -> &'a str {
    let mut bitpos = 0;

    while lines.len() != 1 {
        let one_count = lines
            .iter()
            .map(|line| line.chars().nth(bitpos).unwrap())
            .filter(|&char| char == '1')
            .count();
        let zero_count = lines.len() - one_count;
        let bit_to_keep = match criteria {
            BitCriteria::MostCommon => {
                if one_count >= zero_count {
                    '1'
                } else {
                    '0'
                }
            }
            BitCriteria::LeastCommon => {
                if one_count >= zero_count {
                    '0'
                } else {
                    '1'
                }
            }
        };

        lines.retain(|line| line.chars().nth(bitpos).unwrap() == bit_to_keep);
        bitpos += 1;
    }

    lines[0]
}

pub fn part2<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut oxygen_lines: Vec<&str> = lines
        .into_iter()
        .map(|line| line.as_ref())
        .filter(|line| !line.is_empty())
        .collect();
    let mut co2_lines = oxygen_lines.clone();

    let oxygen_line = part2_filter_by_bit_criteria(&mut oxygen_lines, BitCriteria::MostCommon);
    let co2_line = part2_filter_by_bit_criteria(&mut co2_lines, BitCriteria::LeastCommon);

    let oxygen = u32::from_str_radix(oxygen_line, 2).unwrap();
    let co2 = u32::from_str_radix(co2_line, 2).unwrap();

    Some(oxygen * co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 198);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 230);
    }
}
