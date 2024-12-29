use std::collections::BTreeSet;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct V2 {
    x: usize,
    y: usize,
}

impl V2 {
    fn neighbors(&self, width: usize, height: usize) -> Vec<V2> {
        let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut neighbors = Vec::new();

        for (dx, dy) in deltas {
            let x = (self.x as i32) + dx;
            let y = (self.y as i32) + dy;

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
}

struct Map {
    grid: Grid,
    width: usize,
    height: usize,
    remaining: BTreeSet<V2>,
}

impl Map {
    fn get(&self, v2: &V2) -> char {
        self.grid[v2.y][v2.x]
    }
}

fn parse<'a, I, S>(lines: I) -> Map
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut grid = Vec::new();
    let mut remaining = BTreeSet::new();

    for (y, line) in lines.into_iter().map(|s| s.as_ref()).enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.bytes().map(char::from).enumerate() {
            row.push(c);
            remaining.insert(V2 { x, y });
        }

        grid.push(row);
    }

    let height = grid.len();
    let width = grid[0].len();

    Map {
        grid,
        remaining,
        width,
        height,
    }
}

fn get_plot(map: &mut Map, origin: V2) -> BTreeSet<V2> {
    let plant = map.get(&origin);

    let mut plot = BTreeSet::from([origin]);
    let mut to_check = BTreeSet::from([origin]);

    while let Some(pos_to_check) = to_check.pop_first() {
        let matching_neighbors = pos_to_check
            .neighbors(map.width, map.height)
            .into_iter()
            .filter(|p| !plot.contains(p) && map.get(p) == plant)
            .collect::<Vec<_>>();

        plot.extend(matching_neighbors.iter());
        to_check.extend(matching_neighbors.iter());

        for mn in matching_neighbors.iter() {
            map.remaining.remove(mn);
        }
    }

    plot
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut map = parse(lines);

    let mut count = 0;

    while let Some(origin) = map.remaining.pop_first() {
        let plant = map.get(&origin);
        let plot = get_plot(&mut map, origin);

        let area = plot.len();
        let perimeter: usize = plot
            .iter()
            .map(|pos| {
                let different_neighbors_count = pos
                    .neighbors(map.width, map.height)
                    .iter()
                    .filter(|p| map.get(p) != plant)
                    .count();
                let boundary_count = &[
                    pos.x == 0,
                    pos.y == 0,
                    pos.x == map.width - 1,
                    pos.y == map.height - 1,
                ]
                .iter()
                .filter(|&&b| b)
                .count();
                different_neighbors_count + boundary_count
            })
            .sum();

        count += area * perimeter;
    }

    Ok(count as u32)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
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
        "RRRRIICCFF",
        "RRRRIICCCF",
        "VVRRRCCFFF",
        "VVRCCCJFFF",
        "VVVVCJJCFE",
        "VVIVCCJJEE",
        "VVIIICJJEE",
        "MIIIIIJJEE",
        "MIIISIJEEE",
        "MMMISSJEEE",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 1930);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
