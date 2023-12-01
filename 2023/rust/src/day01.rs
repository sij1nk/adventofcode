use anyhow::{anyhow, Ok};

enum ParsingMode {
    Simple,
    SpelledOutDigits,
}

enum FindDirection {
    Front,
    Back,
}

const DIGITS_MAP: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(chars: &mut impl Iterator<Item = char>) -> Option<u32> {
    for c in chars {
        let maybe_digit = c.to_digit(10);
        if maybe_digit.is_some() {
            return maybe_digit;
        }
    }

    None
}

fn find_first_maybe_spelled_out_digit(mut line: &str, dir: FindDirection) -> Option<u32> {
    loop {
        let mut chars = line.chars();
        let char = match dir {
            FindDirection::Front => chars.next(),
            FindDirection::Back => chars.last(),
        };
        let maybe_digit = char?.to_digit(10);
        if maybe_digit.is_some() {
            return maybe_digit;
        }

        for &(spelled_out_digit, value) in DIGITS_MAP {
            match dir {
                FindDirection::Front => {
                    if line.starts_with(spelled_out_digit) {
                        return Some(value);
                    }
                }
                FindDirection::Back => {
                    if line.ends_with(spelled_out_digit) {
                        return Some(value);
                    }
                }
            }
        }

        line = match dir {
            FindDirection::Front => &line[1..],
            FindDirection::Back => &line[..line.len() - 1],
        }
    }
}

fn get_calibration_value(line: &str, mode: ParsingMode) -> anyhow::Result<u32> {
    let (first_digit, second_digit) = match mode {
        ParsingMode::Simple => {
            let mut chars = line.chars();
            let d1 = find_first_digit(&mut chars)
                .ok_or(anyhow!("Did not find any digits in the line"))?;
            let mut reverse_chars = chars.rev();
            let d2 = find_first_digit(&mut reverse_chars).unwrap_or(d1);

            (d1, d2)
        }
        ParsingMode::SpelledOutDigits => {
            let d1 = find_first_maybe_spelled_out_digit(line, FindDirection::Front)
                .ok_or(anyhow!("Did not find any digits in the line"))?;
            let d2 = find_first_maybe_spelled_out_digit(line, FindDirection::Back)
                .ok_or(anyhow!("Did not find any digits in the line"))?;

            (d1, d2)
        }
    };

    let number = first_digit * 10 + second_digit;
    Ok(number)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            continue;
        }

        sum += get_calibration_value(line, ParsingMode::Simple)?;
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            continue;
        }

        sum += get_calibration_value(line, ParsingMode::SpelledOutDigits)?;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    static EXAMPLE2: &[&str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 142);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE2).unwrap();

        assert_eq!(result, 281);
    }
}
