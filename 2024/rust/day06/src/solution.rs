use std::{collections::BTreeSet, iter::Peekable};

#[derive(Debug, Clone)]
enum Tile {
    Space,
    Obstruction,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct V2 {
    x: i32,
    y: i32,
}

fn get_deltas_iter() -> Peekable<impl Iterator<Item = V2>> {
    let deltas = vec![
        V2 { x: 0, y: -1 },
        V2 { x: 1, y: 0 },
        V2 { x: 0, y: 1 },
        V2 { x: -1, y: 0 },
    ];
    deltas.into_iter().cycle().peekable()
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Map {
    fn get_tile(&self, pos: &V2) -> &Tile {
        &self.tiles[pos.y as usize][pos.x as usize]
    }

    fn with_new_obstruction(&self, pos: &V2) -> Self {
        let mut new_map = self.clone();
        new_map.tiles[pos.y as usize][pos.x as usize] = Tile::Obstruction;

        new_map
    }
}

fn parse<'a, I, S>(lines: I) -> (Map, V2)
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut tiles = Vec::new();
    let mut guard_position = None;

    for (y, line) in lines.into_iter().map(|s| s.as_ref()).enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.bytes().map(char::from).enumerate() {
            let tile = match c {
                '.' => Tile::Space,
                '#' => Tile::Obstruction,
                '^' => {
                    guard_position = Some(V2 {
                        x: x as i32,
                        y: y as i32,
                    });
                    Tile::Space
                }
                _ => panic!("Found unrecognized tile type"),
            };
            row.push(tile);
        }

        tiles.push(row);
    }

    let height = tiles.len() as i32;
    let width = tiles[0].len() as i32;

    let map = Map {
        tiles,
        width,
        height,
    };

    (map, guard_position.expect("Guard should exist"))
}

fn get_next_position(map: &Map, current_pos: &V2, delta: &V2) -> Option<V2> {
    let pos = V2 {
        x: current_pos.x + delta.x,
        y: current_pos.y + delta.y,
    };

    let out_of_bounds = pos.x < 0 || pos.x >= map.width || pos.y < 0 || pos.y >= map.height;
    if out_of_bounds {
        return None;
    }

    Some(pos)
}

fn get_visited_positions(map: &Map, mut guard_pos: V2) -> BTreeSet<V2> {
    let mut deltas_iter = get_deltas_iter();
    let mut current_delta = deltas_iter.next().unwrap();

    let mut visited_positions = BTreeSet::from([guard_pos]);

    while let Some(pos) = get_next_position(map, &guard_pos, &current_delta) {
        match map.get_tile(&pos) {
            Tile::Obstruction => {
                current_delta = deltas_iter.next().unwrap();
            }
            Tile::Space => {
                guard_pos = pos;
                visited_positions.insert(pos);
            }
        }
    }

    visited_positions
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (map, guard_pos) = parse(lines);
    Ok(get_visited_positions(&map, guard_pos).len() as u32)
}

fn try_obstruction_candidate(map: Map, mut current_pos: V2) -> bool {
    let mut deltas_iter = get_deltas_iter();
    let mut current_delta = deltas_iter.next().unwrap();

    let mut visited_positions_with_deltas = BTreeSet::from([(current_pos, current_delta)]);

    while let Some(pos) = get_next_position(&map, &current_pos, &current_delta) {
        match map.get_tile(&pos) {
            Tile::Obstruction => {
                current_delta = deltas_iter.next().unwrap();
            }
            Tile::Space => {
                current_pos = pos;
                let current_pos_with_delta = (current_pos, current_delta);
                if visited_positions_with_deltas.contains(&current_pos_with_delta) {
                    return true;
                } else {
                    visited_positions_with_deltas.insert(current_pos_with_delta);
                }
            }
        }
    }

    false
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (map, guard_pos) = parse(lines);
    let mut visited_positions = get_visited_positions(&map, guard_pos);

    visited_positions.remove(&guard_pos);

    Ok(visited_positions
        .iter()
        .map(|vp| {
            let new_map = map.with_new_obstruction(vp);
            try_obstruction_candidate(new_map, guard_pos)
        })
        .filter(|&b| b)
        .count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 41);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 6);
    }
}
