use std::collections::{BTreeSet, VecDeque};

type Grid = Vec<Vec<u8>>;

#[derive(Debug, Clone)]
struct Map {
    grid: Grid,
    width: usize,
    height: usize,
    trailheads: Vec<V2>,
}

impl Map {
    fn get(&self, v2: &V2) -> u8 {
        self.grid[v2.y][v2.x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct V2 {
    x: usize,
    y: usize,
}

fn parse<'a, I, S>(lines: I) -> Map
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut grid = Vec::new();
    let mut trailheads = Vec::new();

    for (y, line) in lines.into_iter().map(|s| s.as_ref()).enumerate() {
        let mut row = Vec::new();

        for (x, c) in line
            .bytes()
            .map(|b| char::from(b).to_digit(10).unwrap() as u8)
            .enumerate()
        {
            row.push(c);

            if c == 0 {
                trailheads.push(V2 { x, y });
            }
        }

        grid.push(row);
    }

    let width = grid[0].len();
    let height = grid.len();

    Map {
        grid,
        width,
        height,
        trailheads,
    }
}

fn get_neighbors(pos: &V2, width: usize, height: usize) -> Vec<V2> {
    let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut neighbors = Vec::new();

    for (dx, dy) in deltas {
        let x = (pos.x as i32) + dx;
        let y = (pos.y as i32) + dy;

        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) {
            continue;
        }

        neighbors.push(V2 {
            x: x as usize,
            y: y as usize,
        });
    }
    neighbors
}

fn calculate_trailhead_score(map: &Map, trailhead: &V2) -> u32 {
    let mut neighbors = VecDeque::from([*trailhead]);
    let mut covered: BTreeSet<V2> = BTreeSet::from([*trailhead]);
    let mut peaks: BTreeSet<V2> = BTreeSet::new();

    while let Some(pos) = neighbors.pop_back() {
        let current = map.get(&pos);

        if current == 9 {
            peaks.insert(pos);
        }

        let valid_neighbors = get_neighbors(&pos, map.width, map.height)
            .iter()
            .filter(|&neighbor| map.get(neighbor) == current + 1 && !covered.contains(neighbor))
            .copied()
            .collect::<Vec<_>>();

        for valid_n in valid_neighbors {
            covered.insert(valid_n);
            neighbors.push_front(valid_n);
        }
    }

    peaks.len() as u32
}

fn calculate_trailhead_rating(map: &Map, trailhead: &V2) -> u32 {
    let mut neighbors = VecDeque::from([*trailhead]);
    let mut peaks: Vec<V2> = Vec::new();

    while let Some(pos) = neighbors.pop_back() {
        let current = map.get(&pos);

        if current == 9 {
            peaks.push(pos);
        }

        let valid_neighbors = get_neighbors(&pos, map.width, map.height)
            .iter()
            .filter(|&neighbor| map.get(neighbor) == current + 1)
            .copied()
            .collect::<Vec<_>>();

        for valid_n in valid_neighbors {
            neighbors.push_front(valid_n);
        }
    }

    peaks.len() as u32
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = parse(lines);

    Ok(map
        .trailheads
        .iter()
        .map(|th| calculate_trailhead_score(&map, th))
        .sum())
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = parse(lines);

    Ok(map
        .trailheads
        .iter()
        .map(|th| calculate_trailhead_rating(&map, th))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
        "10456732",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 36);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 81);
    }
}
