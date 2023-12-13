use std::fmt;

#[derive(Default)]
struct Map<'a> {
    inner: Vec<&'a str>,
}

impl<'a> fmt::Debug for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &row in self.inner.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

fn columns_are_mirrored_at(map: &Map, i: usize) -> bool {
    let inner = &map.inner;
    let row_len = inner[0].len();

    let column_pairs_to_compare = i.min(row_len - i - 2);

    for dist in 0..column_pairs_to_compare {
        let column = inner.iter().map(|&row| row.chars().nth(i - (dist + 1)));
        let next_column = inner.iter().map(|&row| row.chars().nth(i + 1 + dist + 1));

        if !column.zip(next_column).all(|(c1, c2)| c1 == c2) {
            return false;
        }
    }

    true
}

fn rows_are_mirrored_at(map: &Map, i: usize) -> bool {
    let inner = &map.inner;
    let column_len = inner.len();

    let row_pairs_to_compare = i.min(column_len - i - 2);

    for dist in 0..row_pairs_to_compare {
        let row = inner[i - (dist + 1)];
        let next_row = inner[i + 1 + dist + 1];

        if row != next_row {
            return false;
        }
    }

    true
}

fn solve_by_rows(map: &Map) -> Option<u32> {
    let inner = &map.inner;

    for (i, (row, next_row)) in inner.iter().zip(inner.iter().skip(1)).enumerate() {
        let maybe_mirror = row == next_row;

        if maybe_mirror && rows_are_mirrored_at(map, i) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn solve_by_columns(map: &Map) -> Option<u32> {
    let inner = &map.inner;
    let row_len = inner[0].len();

    let mut i = 0;

    while i + 1 < row_len {
        let column = inner.iter().map(|&row| row.chars().nth(i));
        let next_column = inner.iter().map(|&row| row.chars().nth(i + 1));

        let maybe_mirror = column.zip(next_column).all(|(c1, c2)| c1 == c2);
        if maybe_mirror && columns_are_mirrored_at(map, i) {
            return Some(i as u32 + 1);
        }

        i += 1;
    }

    None
}

fn solve(map: &Map) -> anyhow::Result<u32> {
    let column_score = solve_by_columns(map).unwrap_or_default();
    let row_score = solve_by_rows(map).unwrap_or_default();

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
            sum += solve(&map)?;
            map.inner.clear();
        } else {
            map.inner.push(line);
        }
    }

    // Input doesn't have a blank line at the end
    sum += solve(&map)?;

    Ok(sum)
}

pub fn part2<'a, I, S>(_lines: I) -> anyhow::Result<u32>
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

        assert_eq!(result, 0);
    }
}
