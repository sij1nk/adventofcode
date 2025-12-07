use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Paper,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            crap => panic!("Unrecognized input: {}", crap),
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    // using a 2d vec as opposed to a hashset gives a ~6x speedup
    tiles: Vec<Vec<Tile>>,
    w: i32,
    h: i32,
}

impl World {
    fn get_tile(&self, x: i32, y: i32) -> Tile {
        self.tiles[y as usize][x as usize]
    }
}

fn parse_world<'a, I, S>(lines: I) -> anyhow::Result<World>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut tiles = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let row = line.chars().map(Tile::from).collect::<Vec<_>>();
        tiles.push(row);
    }

    let w = tiles[0].len() as i32;
    let h = tiles.len() as i32;

    Ok(World { tiles, w, h })
}

fn get_neighbors_count(world: &World, x: i32, y: i32) -> usize {
    let dxdy = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    dxdy.iter()
        .filter(|(dx, dy)| {
            let xx = x + dx;
            let yy = y + dy;

            if !(0 <= xx && xx < world.w && 0 <= yy && yy < world.h) {
                return false;
            }

            world.get_tile(xx, yy) == Tile::Paper
        })
        .count()
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let world = parse_world(lines)?;
    let mut count = 0;

    for y in 0..world.h {
        for x in 0..world.w {
            let is_paper = world.get_tile(x, y) == Tile::Paper;
            if is_paper && get_neighbors_count(&world, x, y) < 4 {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut world = parse_world(lines)?;
    let mut removed = 0;

    loop {
        let mut to_remove = HashSet::new();

        // this seems really slow, since we iterate over empty tiles multiple times, but it's
        // substantially faster than storing just the paper coordinates in a set
        //
        // I tried using both a 2d vec and a set, and using the vec only for the lookups.
        // It's not as slow, but still way slower
        for y in 0..world.h {
            for x in 0..world.w {
                let is_paper = world.get_tile(x, y) == Tile::Paper;
                if is_paper && get_neighbors_count(&world, x, y) < 4 {
                    to_remove.insert((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        removed += to_remove.len();

        for &(x, y) in to_remove.iter() {
            world.tiles[y as usize][x as usize] = Tile::Empty;
        }

        to_remove.clear();
    }

    Ok(removed as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 43);
    }
}
