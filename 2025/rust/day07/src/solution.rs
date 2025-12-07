use std::collections::{BTreeMap, BTreeSet};

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut prev: BTreeSet<usize> = BTreeSet::new();
    let mut curr: BTreeSet<usize> = BTreeSet::new();

    let mut splits = 0;

    // every 2nd line in the input is padding which we can skip
    let mut lines = lines.into_iter().step_by(2).map(|l| l.as_ref());

    let first_line = lines.next().expect("First line exists");

    let (start, _) = first_line
        .chars()
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .expect("Starting position exists");
    prev.insert(start);

    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        for &pc in prev.iter() {
            let c = chars[pc];
            if c == '.' {
                curr.insert(pc);
            } else if c == '^' {
                splits += 1;
                // there is never a splitter next to the edge, so we don't need to check bounds
                curr.insert(pc - 1);
                curr.insert(pc + 1);
            }
        }

        prev = curr.clone();
        curr.clear();
    }

    Ok(splits)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut prev: BTreeMap<usize, u64> = BTreeMap::new();
    let mut curr: BTreeMap<usize, u64> = BTreeMap::new();

    // every 2nd line in the input is padding which we can skip
    let mut lines = lines.into_iter().step_by(2).map(|l| l.as_ref());

    let first_line = lines.next().expect("First line exists");

    let (start, _) = first_line
        .chars()
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .expect("Starting position exists");
    prev.insert(start, 1);

    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        for (&i, &n) in prev.iter() {
            let c = chars[i];
            if c == '.' {
                *curr.entry(i).or_default() += n;
            } else if c == '^' {
                // there is never a splitter next to the edge, so we don't need to check bounds
                *curr.entry(i - 1).or_default() += n;
                *curr.entry(i + 1).or_default() += n;
            }
        }

        prev = curr.clone();
        curr.clear();
    }

    Ok(prev.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 21);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 40);
    }
}
