#[derive(Default)]
struct Map<'a> {
    inner: Vec<&'a str>,
}

fn get_differing_chars_in_lines(
    line1: impl Iterator<Item = char>,
    line2: impl Iterator<Item = char>,
) -> u32 {
    line1
        .zip(line2)
        .fold(0, |acc, (c1, c2)| if c1 != c2 { acc + 1 } else { acc })
}

fn columns_are_mirrored_at(map: &Map, i: usize, consider_smudges: bool) -> bool {
    let inner = &map.inner;
    let row_len = inner[0].len();
    let mut found_smudge = false;

    let column_pairs_to_compare = i.min(row_len - i - 2);

    for dist in 0..column_pairs_to_compare {
        let column = inner
            .iter()
            .filter_map(|&row| row.chars().nth(i - (dist + 1)));
        let next_column = inner
            .iter()
            .filter_map(|&row| row.chars().nth(i + 1 + dist + 1));

        let diffs = get_differing_chars_in_lines(column, next_column);

        if consider_smudges {
            if diffs == 1 {
                if !found_smudge {
                    found_smudge = true;
                } else {
                    return false;
                }
            } else if diffs != 0 {
                return false;
            }
        } else if diffs != 0 {
            return false;
        }
    }

    if consider_smudges {
        found_smudge
    } else {
        true
    }
}

fn rows_are_mirrored_at(map: &Map, i: usize, consider_smudges: bool) -> bool {
    let inner = &map.inner;
    let column_len = inner.len();
    let mut found_smudge = false;

    let row_pairs_to_compare = i.min(column_len - i - 2);

    for dist in 0..row_pairs_to_compare {
        let row = inner[i - (dist + 1)];
        let next_row = inner[i + 1 + dist + 1];

        let diffs = get_differing_chars_in_lines(row.chars(), next_row.chars());

        if consider_smudges {
            if diffs == 1 {
                if !found_smudge {
                    found_smudge = true;
                } else {
                    return false;
                }
            } else if diffs != 0 {
                return false;
            }
        } else if diffs != 0 {
            return false;
        }
    }

    if consider_smudges {
        found_smudge
    } else {
        true
    }
}

fn solve_by_rows(map: &Map, consider_smudges: bool) -> Option<u32> {
    let inner = &map.inner;

    for (i, (row, next_row)) in inner.iter().zip(inner.iter().skip(1)).enumerate() {
        let diffs = get_differing_chars_in_lines(row.chars(), next_row.chars());
        if consider_smudges {
            if (diffs == 0 && rows_are_mirrored_at(map, i, true))
                || (diffs == 1 && rows_are_mirrored_at(map, i, false))
            {
                return Some(i as u32 + 1);
            }
        } else if diffs == 0 && rows_are_mirrored_at(map, i, false) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn solve_by_columns(map: &Map, consider_smudges: bool) -> Option<u32> {
    let inner = &map.inner;
    let row_len = inner[0].len();

    let mut i = 0;

    while i + 1 < row_len {
        let column = inner.iter().filter_map(|&row| row.chars().nth(i));
        let next_column = inner.iter().filter_map(|&row| row.chars().nth(i + 1));
        let diffs = get_differing_chars_in_lines(column, next_column);

        if consider_smudges {
            if (diffs == 0 && columns_are_mirrored_at(map, i, true))
                || (diffs == 1 && columns_are_mirrored_at(map, i, false))
            {
                return Some(i as u32 + 1);
            }
        } else if diffs == 0 && columns_are_mirrored_at(map, i, false) {
            return Some(i as u32 + 1);
        }

        i += 1;
    }

    None
}

fn solve(map: &Map, consider_smudges: bool) -> anyhow::Result<u32> {
    let column_score = solve_by_columns(map, consider_smudges).unwrap_or_default();
    let row_score = solve_by_rows(map, consider_smudges).unwrap_or_default();

    Ok(column_score + 100 * row_score)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;
    let mut map = Map::default();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            sum += solve(&map, false)?;
            map.inner.clear();
        } else {
            map.inner.push(line);
        }
    }

    // Input doesn't have a blank line at the end
    sum += solve(&map, false)?;

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;
    let mut map = Map::default();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            sum += solve(&map, true)?;
            map.inner.clear();
        } else {
            map.inner.push(line);
        }
    }

    // Input doesn't have a blank line at the end
    sum += solve(&map, true)?;

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    static EXAMPLE_2: &[&str] = &[
        "##.###.##.###.###",
        "#...########...##",
        ".###........###..",
        ".#..########..#..",
        "#.#.######.#.#.##",
        "#....######....##",
        "#####..##..######",
        ".#..#.####.#..#..",
        "##.#........#.###",
        ".#..########..#..",
        "######.##.#######",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();
        let result2 = part1(EXAMPLE_2).unwrap();

        assert_eq!(result, 405);
        assert_eq!(result2, 16);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 400);
    }
}
