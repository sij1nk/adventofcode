use ascii::AsciiChar;
use itertools::Itertools;

fn item_priority(c: char) -> Option<u8> {
    let ascii_c = AsciiChar::from_ascii(c).ok()?;
    if !ascii_c.is_alphabetic() {
        return None;
    }

    if ascii_c.is_uppercase() {
        Some(ascii_c.as_byte() - 38)
    } else {
        Some(ascii_c.as_byte() - 96)
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut priority: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let len = line.len();

        let chars = line.chars().collect::<Vec<_>>();
        let (first_half, second_half) = chars.split_at(len / 2);

        for second_half_char in second_half.iter() {
            if first_half.iter().any(|c| c == second_half_char) {
                if let Some(prio) = item_priority(*second_half_char) {
                    priority += u32::from(prio);
                }
                break;
            }
        }

    }

    Some(priority)
}

pub fn part2<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut priority: u32 = 0;

    lines.into_iter().map(|l| l.as_ref().chars()).chunks(3).into_iter().for_each(|mut group| {
        let mut common = vec![];
        let first_line_chars = group.next().unwrap().collect::<Vec<_>>();

        for second_line_char in group.next().unwrap() {
            if first_line_chars.contains(&second_line_char) {
                common.push(second_line_char);
            }
        }

        let badge = group.next().unwrap().find(|&c| common.contains(&c)).unwrap();
        priority += u32::from(item_priority(badge).unwrap());
    });

    Some(priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 157);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 70);
    }
}
