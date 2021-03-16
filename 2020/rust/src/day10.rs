use crate::util;
use std::collections::BTreeMap;
use std::error::Error;

pub fn part1<'a, I, S>(lines: I) -> Result<u32, Box<dyn Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut ones = 0;
    let mut threes = 1;
    let mut nums: Vec<u32> = util::parse_many(lines)?;
    nums.insert(0, 0);
    nums.sort_unstable();
    for (n1, n2) in nums.iter().zip(nums.iter().skip(1)) {
        match n2 - n1 {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    Ok(ones * threes)
}

pub fn part2<'a, I, S>(lines: I) -> Result<usize, Box<dyn Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut routes: BTreeMap<u32, Vec<&u32>> = BTreeMap::new();
    let mut stack: BTreeMap<u32, usize> = BTreeMap::new();
    let mut nums: Vec<u32> = util::parse_many(lines)?;
    stack.insert(*nums.iter().max().unwrap(), 1);
    nums.insert(0, 0);
    nums.sort_unstable();

    for num in 1..nums.len() {
        routes.insert(
            nums[num],
            nums[num.checked_sub(3).unwrap_or(0)..num]
                .iter()
                .filter(|&n| nums[num] - n <= 3)
                .collect(),
        );
    }

    loop {
        let highest_key = *stack.keys().max().unwrap();
        let (num, multiplier) = stack.remove_entry(&highest_key).unwrap();
        if num == 0 {
            return Ok(multiplier);
        }

        for route in routes.get(&num).unwrap().iter() {
            if stack.contains_key(&route) {
                if let Some(existing_multiplier) = stack.get_mut(&route) {
                    *existing_multiplier += multiplier;
                }
            } else {
                stack.insert(**route, multiplier);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 220);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 19208);
    }
}
