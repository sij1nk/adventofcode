use anyhow::anyhow;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Spring {
    Damaged,
    Unknown,
}

type Segment = Vec<Spring>;

// Operational springs act as separators between spring segments containing only Damaged or Unknown
// springs
fn parse_line(line: &str) -> anyhow::Result<(Vec<Segment>, Vec<i32>)> {
    let (segments_str, groups_str) = line
        .split_once(' ')
        .ok_or(anyhow!("Couldn't split line on whitespace"))?;

    let segments = segments_str
        .split('.')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Ok(Spring::Damaged),
                    '?' => Ok(Spring::Unknown),
                    unknown => Err(anyhow!("Unexpected character in input: '{}'", unknown)),
                })
                .collect::<Result<Segment, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let groups = groups_str
        .split(',')
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((segments, groups))
}

// We cannot leave Damaged springs behind in the current segment!
fn skipped_damaged_springs(segment: &Segment, i: usize) -> bool {
    let Some(range) = segment.get(0..i.saturating_sub(1)) else {
        return false;
    };

    range.contains(&Spring::Damaged)
}

// Start from the chosen index and expand in both directions until segment bounds or an Unknown
// spring is hit
fn get_damaged_range_bounds(segment: &Segment, i: usize, group_size: i32) -> (i32, i32) {
    let mut start = i as i32;
    let mut end = i as i32 + (group_size - 1);
    let mut reached_beginning = false;
    let mut reached_end = false;
    let mut dist = 1;

    loop {
        if reached_beginning && reached_end {
            break;
        }

        if !reached_beginning {
            start -= dist;
            if start < 0 {
                start = 0;
                reached_beginning = true;
            } else if segment[start as usize] == Spring::Unknown {
                start += 1;
                reached_beginning = true;
            }
        }

        if !reached_end {
            end += dist;
            if end >= segment.len() as i32 || segment[end as usize] == Spring::Unknown {
                reached_end = true;
            }
        }

        dist += 1;
    }

    // +1 because we need to have Operational springs inbetween groups of Damaged ones
    (start, end)
}

fn solve(segments: &Vec<Segment>, groups: &[i32]) -> anyhow::Result<i32> {
    // We reached the end
    if groups.is_empty() {
        return if segments
            .iter()
            .any(|seg| seg.iter().any(|&s| s == Spring::Damaged))
        {
            // ...but there are still Damaged springs left!
            Ok(0)
        } else {
            Ok(1)
        };
    }

    let mut sum = 0;
    let group_size = groups[0];
    let mut stop = false;

    for (segment_i, segment) in segments.iter().enumerate() {
        if stop {
            break;
        }

        // We cannot leave Damaged springs behind in previous segments!
        stop = segment.iter().any(|&s| s == Spring::Damaged);

        for i in 0..segment.len().saturating_sub((group_size - 1) as usize) {
            if skipped_damaged_springs(segment, i) {
                break;
            }

            let (damaged_group_start, damaged_group_end) =
                get_damaged_range_bounds(segment, i, group_size);
            if damaged_group_end - damaged_group_start != group_size {
                continue;
            }

            let mut next_segments = segments.clone();
            for _ in 0..segment_i {
                next_segments.remove(0);
            }

            if damaged_group_end == next_segments[0].len() as i32 {
                next_segments.remove(0);
            } else {
                let next_first_segment = next_segments
                    .get_mut(0)
                    .expect("There must always be a next first segment - the current one");
                *next_first_segment =
                    next_first_segment[(damaged_group_end + 1) as usize..].to_vec();
            }

            sum += solve(&next_segments, &groups[1..])?;
        }
    }

    Ok(sum)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<i32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (segments, groups) = parse_line(line)?;
        sum += solve(&segments, &groups)?;
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(_lines: I) -> anyhow::Result<i32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 21);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 525152);
    }
}
