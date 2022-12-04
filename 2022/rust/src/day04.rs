use std::str::FromStr;

use crate::util;

struct Pair {
    elf1_start: u32,
    elf1_end: u32,
    elf2_start: u32,
    elf2_end: u32
}

impl Pair {
    fn overlap(&self) -> bool {
        self.elf2_start <= self.elf1_end && self.elf1_start <= self.elf2_end
    }

    fn fully_overlap(&self) -> bool {
        (self.elf1_start <= self.elf2_start && self.elf2_end <= self.elf1_end) || 
            (self.elf2_start <= self.elf1_start && self.elf1_end <= self.elf2_end)
    }

    fn parse_range(range: &str) -> Result<(u32, u32), util::Error> {
       let mut range = range 
            .split('-')
            .map(|s| s.parse::<u32>().map_err(|_| util::Error::new("Failed to parse range start/end as number")))
            .take(2)
            .collect::<Result<Vec<_>, util::Error>>()?.into_iter();

        let start = range.next().ok_or_else(|| util::Error::new("Range start number missing"))?;
        let end = range.next().ok_or_else(|| util::Error::new("Range end number missing"))?;

        if range.next().is_some() {
            Err(util::Error::new("Parsed more than 2 numbers for the range"))
        } else {
            Ok((start, end))
        }
    }
}

impl FromStr for Pair {
    type Err = util::Error;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (elf1, elf2) = s.split_once(',').ok_or_else(|| util::Error::new("Separator ',' not found"))?;

        let (elf1_start, elf1_end) = Pair::parse_range(elf1)?;
        let (elf2_start, elf2_end) = Pair::parse_range(elf2)?;

        Ok(Pair { elf1_start, elf2_start, elf1_end, elf2_end})
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<u32, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut count = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let pair = Pair::from_str(line)?;
        if pair.fully_overlap() {
            count += 1;
        }
    }

    Ok(count)
}

pub fn part2<'a, I, S>(lines: I) -> Result<u32, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut count = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let pair = Pair::from_str(line)?;
        if pair.overlap() {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 4);
    }
}
