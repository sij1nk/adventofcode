use std::collections::BTreeMap;

use itertools::Itertools;

type Int = u64;
type Blinks = usize;
type Cache = BTreeMap<Int, Vec<Int>>;

fn get_digits_count(n: Int) -> Int {
    n.ilog10() as Int + 1
}

fn blink(n: Int) -> Vec<Int> {
    if n == 0 {
        return vec![1];
    };

    let digits_count = get_digits_count(n);
    if digits_count % 2 == 0 {
        let half_digits = digits_count / 2;
        let divisor = (10 as Int).pow(half_digits as u32);

        let last_two = n % divisor;
        let first_two = n / divisor;

        return vec![first_two, last_two];
    }

    vec![n * 2024]
}

fn solve<'a, I, S>(lines: I, blink_count: Blinks) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut cache: Cache = BTreeMap::new();

    let mut stones: BTreeMap<Int, usize> = lines
        .into_iter()
        .map(|s| s.as_ref().split(' ').map(|w| w.parse::<Int>().unwrap()))
        .next()
        .unwrap()
        .dedup_with_count()
        .map(|(k, v)| (v, k))
        .collect();

    for _ in 0..blink_count {
        let mut intermediate_results: BTreeMap<Int, usize> = BTreeMap::new();

        for (num, amount) in stones.iter() {
            let results = cache.entry(*num).or_insert(blink(*num));
            for result in results.iter() {
                *intermediate_results.entry(*result).or_default() += amount;
            }
        }

        stones = intermediate_results;
    }

    stones.values().sum()
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(solve(lines, 25))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(solve(lines, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["125 17"];

    #[test]
    fn blink_works() {
        let nums: Vec<Int> = vec![0, 1, 2, 99, 2024];

        let result = nums.into_iter().flat_map(blink).collect::<Vec<_>>();

        assert_eq!(result, vec![1, 2024, 4048, 9, 9, 20, 24]);
    }

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 55312);
    }
}
