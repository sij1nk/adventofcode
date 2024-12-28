use std::collections::{BTreeMap, VecDeque};

type Int = u64;
type Blinks = usize;
type Cache = BTreeMap<Int, BTreeMap<Blinks, Vec<Int>>>;

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
fn populate_cache(blink_count: Blinks) -> Cache {
    let mut cache = BTreeMap::new();

    for n in 0..10 {
        let n_cache: &mut BTreeMap<Blinks, Vec<Int>> = cache.entry(n).or_default();

        let mut stones = vec![n];
        for i in 0..blink_count {
            let blinks = i + 1;
            stones = stones.into_iter().flat_map(blink).collect::<Vec<_>>();
            n_cache.insert(blinks, stones.clone());
        }
    }

    cache
}

fn solve<'a, I, S>(lines: I, blink_count: Blinks) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut count = 0;
    let mut cache: Cache = populate_cache(15);

    let mut stones = lines
        .into_iter()
        .map(|s| {
            s.as_ref()
                .split(' ')
                .map(|w| w.parse::<Int>().unwrap())
                .map(|n| (blink_count as Blinks, n))
        })
        .next()
        .unwrap()
        .collect::<VecDeque<_>>();

    while let Some((rem_blinks, n)) = stones.pop_back() {
        let n_cache = cache.entry(n).or_default();
        let cache_result = n_cache
            .iter()
            .filter(|&(&k, _)| k <= rem_blinks)
            .next_back();

        let (rem_blinks_after, blink_result) = match cache_result {
            Some((best_key, blink_result)) => (rem_blinks - best_key, blink_result.clone()),
            None => {
                let blink_result = blink(n);
                (rem_blinks - 1, blink_result)
            }
        };

        if rem_blinks_after == 0 {
            count += blink_result.len();
        } else {
            for n in blink_result {
                stones.push_front((rem_blinks_after, n));
            }
        }
    }

    count
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
    Ok(solve(lines, 25))
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

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
