use std::collections::VecDeque;

use itertools::Itertools;

pub fn part1<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let line = lines.into_iter().map(|l| l.as_ref()).next().unwrap();
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(4);

    for (i, c) in line.chars().enumerate() {
        if buffer.len() < 4 {
            buffer.push_back(c);
        } else if buffer.iter().all_unique() {
            return Some(i);
        } else {
            buffer.pop_front();
            buffer.push_back(c);
        }
    }

    None
}

pub fn part2<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let line = lines.into_iter().map(|l| l.as_ref()).next().unwrap();
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(4);

    for (i, c) in line.chars().enumerate() {
        if buffer.len() < 14 {
            buffer.push_back(c);
        } else if buffer.iter().all_unique() {
            return Some(i);
        } else {
            buffer.pop_front();
            buffer.push_back(c);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 11);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 26);
    }
}
