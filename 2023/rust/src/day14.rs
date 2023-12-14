use anyhow::anyhow;
use std::fmt;

#[derive(Clone, Copy)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> anyhow::Result<Self> {
        match c {
            'O' => Ok(Tile::Round),
            '#' => Ok(Tile::Cube),
            '.' => Ok(Tile::Empty),
            unknown => Err(anyhow!("Unknown tile in input: '{}", unknown)),
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Round => write!(f, "O"),
            Self::Cube => write!(f, "#"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Default)]
struct Map {
    inner: Vec<Vec<Tile>>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner.iter() {
            for tile in row {
                write!(f, "{:?}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn calculate_weight(level: u32, count: u32) -> u32 {
    (level - count..level).sum::<u32>()
}

fn solve_line(line: &Vec<Tile>) -> u32 {
    let len = line.len();
    let mut sum = 0;
    let mut level = len + 1;
    let mut tile_count = 0;

    for (i, tile) in line.iter().enumerate() {
        match tile {
            Tile::Round => {
                tile_count += 1;
            }
            Tile::Cube => {
                if tile_count > 0 {
                    sum += calculate_weight(level as u32, tile_count);
                }
                level = len - i;
                tile_count = 0;
            }
            Tile::Empty => {}
        }
    }

    if tile_count > 0 {
        sum += calculate_weight(level as u32, tile_count);
    }

    sum
}

fn transpose_map(map: &Map) -> Map {
    let column_count = map.inner.len();
    let column_len = map.inner[0].len();
    let mut columns: Vec<Vec<Tile>> = Vec::with_capacity(column_count);
    for _ in 0..column_count {
        columns.push(Vec::with_capacity(column_len));
    }

    for rows in map.inner.iter() {
        for (y, &c) in rows.iter().enumerate() {
            columns[y].push(c);
        }
    }

    Map { inner: columns }
}

fn solve(map: &Map) -> u32 {
    let transposed = transpose_map(map);
    transposed.inner.iter().map(solve_line).sum()
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut map = Map::default();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c)?);
        }

        map.inner.push(row);
    }

    Ok(solve(&map))
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
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 136);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
