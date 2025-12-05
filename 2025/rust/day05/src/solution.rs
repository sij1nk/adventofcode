use anyhow::anyhow;
use std::collections::BTreeSet;

fn part1_optimize_ranges(ranges: &BTreeSet<(u64, u64)>) -> BTreeSet<(u64, u64)> {
    let mut opt = BTreeSet::new();

    let first = ranges.first().unwrap();
    let mut curr_x = first.0;
    let mut curr_y = first.1;

    for (x, y) in ranges.iter().skip(1).copied() {
        if x > curr_y {
            opt.insert((curr_x, curr_y));
            curr_x = x;
            curr_y = y;
        }
        curr_y = curr_y.max(y);
    }

    opt.insert((curr_x, curr_y));

    opt
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut ranges: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut parse_ids = false;

    let mut count = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            parse_ids = true;
            ranges = part1_optimize_ranges(&ranges); // to make id lookups cheaper
            continue;
        }

        if parse_ids {
            let id = line.parse::<u64>()?;

            let r = ranges.iter().find(|(x, y)| *x <= id && id <= *y);
            if r.is_some() {
                count += 1;
            }
        } else {
            // parse ranges
            let (x_str, y_str) = line
                .split_once("-")
                .ok_or_else(|| anyhow!("Could not parse range"))?;
            let x = x_str.parse::<u64>()?;
            let y = y_str.parse::<u64>()?;

            ranges.insert((x, y));
        }
    }

    Ok(count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut ranges: BTreeSet<(u64, u64)> = BTreeSet::new();

    let mut count = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            break;
        }

        let (x_str, y_str) = line
            .split_once("-")
            .ok_or_else(|| anyhow!("Could not parse range"))?;
        let x = x_str.parse::<u64>()?;
        let y = y_str.parse::<u64>()?;

        ranges.insert((x, y));
    }

    let mut max = 0;

    for (&(r1x, r1y), &(r2x, r2y)) in ranges.iter().zip(ranges.iter().skip(1)) {
        let p = (r1y.max(max) + 1).min(r2x) - r1x;
        count += p;

        // we need to "retain" the highest of the two upper bounds for the next iteration, for situations like:
        // (  )
        //   (     ####     )
        //     (  )
        //             (        )
        // part highlighted with # would be skipped otherwise
        let new_max = r1y.max(r2y);
        max = new_max;
    }

    count += ranges.last().map(|(x, y)| y - x + 1).unwrap();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 14);
    }
}
