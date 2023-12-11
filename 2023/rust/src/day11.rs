use std::collections::BTreeSet;

#[derive(Debug, Copy, Clone)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn distance(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// We're parsing row-by-row, so we can expand down during parsing, but we can only figure out
// whether we need to expand right for a particular column if we've parsed the entire map
fn parse_map<'a, I, S>(lines: I, expand_by: u64) -> Vec<Position>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut positions = Vec::new();
    let mut expand_down = 0;

    let lines: Vec<&str> = lines.into_iter().map(|l| l.as_ref()).collect();
    let line_len = lines[0].len() as u64;

    let mut expand_right: BTreeSet<u64> = (0..line_len).collect();

    for (y, line) in lines.iter().enumerate() {
        let mut should_expand_row = true;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                expand_right.remove(&(x as u64));
                positions.push(Position {
                    x: x as u64,
                    y: y as u64 + expand_down,
                });
                should_expand_row = false;
            }
        }

        if should_expand_row {
            expand_down += expand_by;
        }
    }

    for p in positions.iter_mut() {
        let expand = expand_right.iter().filter(|&&x| x < p.x).count() as u64;
        p.x += expand * expand_by;
    }

    positions
}

fn sum_shortest_paths(positions: &[Position]) -> u64 {
    let mut sum = 0;

    for (i, p1) in positions.iter().enumerate() {
        for p2 in positions.iter().skip(i) {
            sum += p1.distance(p2);
        }
    }

    sum
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let positions = parse_map(lines, 1);
    Ok(sum_shortest_paths(&positions))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let positions = parse_map(lines, 999999);
    Ok(sum_shortest_paths(&positions))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 374);
    }

    #[test]
    fn part2_test() {
        let result = sum_shortest_paths(&parse_map(EXAMPLE, 9));
        let result2 = sum_shortest_paths(&parse_map(EXAMPLE, 99));

        assert_eq!(result, 1030);
        assert_eq!(result2, 8410);
    }
}
