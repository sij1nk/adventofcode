use anyhow::anyhow;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet};

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let line = lines
        .into_iter()
        .map(|l| l.as_ref())
        .next()
        .ok_or(anyhow!("Input is missing"))?;

    let mut sum = 0;

    for range in line.split(",") {
        let (start_str, end_str) = range
            .split_once("-")
            .ok_or(anyhow!("Failed to parse range"))?;

        let startlen = start_str.len();
        let endlen = end_str.len();

        if startlen % 2 == 1 && startlen == endlen {
            continue;
        }

        let start = start_str.parse::<u64>()?;
        let end = end_str.parse::<u64>()?;
        let (start_str_half, _) = start_str.split_at(cmp::max(start_str.len() / 2, 1));

        let mut start_half = if start_str_half.len() == 1 {
            // hack for ranges where the start is a single digit (e.g. 2-19)
            // normally `22` would be the first number we check here, which would be out of bounds,
            // and we'd miss `11`
            1
        } else {
            start_str_half.parse::<u64>()?
        };

        loop {
            let n_str = format!("{}{}", start_half, start_half);
            let n = n_str.parse::<u64>()?;

            if n > end {
                break;
            }

            if start <= n {
                sum += n;
            }
            start_half += 1;
        }
    }

    Ok(sum)
}

fn collect_from_range_delim(
    input_str: &str,
    inputlen: usize,
    factors: &BTreeMap<usize, Vec<u64>>,
    start: u64,
    end: u64,
    d: i64,
) -> anyhow::Result<BTreeSet<u64>> {
    let mut results = BTreeSet::new();

    let Some(fs) = factors.get(&inputlen) else {
        return Ok(BTreeSet::new());
    };

    for f in fs {
        let repeat = inputlen as u64 / f;
        let (n_str, _) = input_str.split_at(*f as usize);
        let mut n = n_str.parse::<u64>()?;

        loop {
            let nnn_str = format!("{n}").repeat(repeat as usize);
            let nnn = nnn_str.parse::<u64>()?;

            if nnn > end {
                break;
            }

            if start <= nnn {
                results.insert(nnn);
            }

            n = n.strict_add_signed(d);

            if n == 0 || n.ilog10() as u64 + 1 != *f {
                break;
            }
        }
    }

    Ok(results)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let factors: BTreeMap<usize, Vec<u64>> = BTreeMap::from([
        // no entry for `1` because a single digit (repeating once) is not a valid result
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![1, 2]),
        (5, vec![1]),
        (6, vec![1, 2, 3]),
        (7, vec![1]),
        (8, vec![1, 2, 4]),
        (9, vec![1, 3]),
        (10, vec![1, 2, 5]),
        (11, vec![1]),
        (12, vec![1, 2, 3, 4, 6]),
        (13, vec![1]),
        (14, vec![1, 2, 7]),
        (15, vec![1, 3, 5]),
        (16, vec![1, 2, 4, 8]),
        // ...this should be enough
    ]);

    let line = lines
        .into_iter()
        .map(|l| l.as_ref())
        .next()
        .ok_or(anyhow!("Input is missing"))?;

    let mut results = BTreeSet::new();

    for range in line.split(",") {
        let (start_str, end_str) = range
            .split_once("-")
            .ok_or(anyhow!("Failed to parse range"))?;

        let start = start_str.parse::<u64>()?;
        let end = end_str.parse::<u64>()?;

        let startlen = start_str.len();
        let endlen = end_str.len();

        results.extend(collect_from_range_delim(
            start_str, startlen, &factors, start, end, 1,
        )?);

        if startlen != endlen {
            results.extend(collect_from_range_delim(
                end_str, endlen, &factors, start, end, -1,
            )?);
        }
    }

    Ok(results.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 4174379265);
    }
}
