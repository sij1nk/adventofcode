use crate::util;
use std::error::Error;

pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut count = 0;
    let mut iter = lines.into_iter().map(|item| item.as_ref().parse::<u32>().unwrap());
    let mut last = iter.next()?;

    for num in iter {
        if last < num {
            count += 1;
        }
        last = num
    }

    Some(count)
}

pub fn part2<'a, I, S>(lines: I) -> Result<usize, Box<dyn Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let nums: Vec<u32> = util::parse_many(lines)?;
    
    let windows: Vec<u32> = nums.iter()
        .zip(nums.iter().skip(1))
        .zip(nums.iter().skip(2))
        .map(|((x, y), z)| x+y+z)
        .collect();

    Ok(windows.iter()
        .zip(windows.iter().skip(1))
        .filter(|(w1, w2)| w1 < w2)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["199", "200", "208", "210", "200", "207", "240", "269", "260", "263"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 7);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 5);
    }
}
