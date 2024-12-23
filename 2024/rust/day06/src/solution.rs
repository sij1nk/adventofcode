use std::collections::BTreeSet;

enum Tile {
    Space,
    Obstruction,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct V2 {
    x: i32,
    y: i32,
}

fn get_deltas_iter() -> impl Iterator<Item = V2> {
    let deltas = vec![
        V2 { x: 0, y: -1 },
        V2 { x: 1, y: 0 },
        V2 { x: 0, y: 1 },
        V2 { x: -1, y: 0 },
    ];
    deltas.into_iter().cycle()
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
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
        height,
        width,
    };

    (map, guard_position.expect("Guard should exist"))
}

fn get_next_position(map: &Map, current: &V2, delta: &V2) -> Option<V2> {
    let pos = V2 {
        x: current.x + delta.x,
        y: current.y + delta.y,
    };

    let out_of_bounds = pos.x < 0 || pos.x >= map.width || pos.y < 0 || pos.y >= map.height;
    if out_of_bounds {
        return None;
    }

    Some(pos)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (map, mut guard_pos) = parse(lines);
    let mut deltas_iter = get_deltas_iter();
    let mut current_delta = deltas_iter.next().unwrap();

    let mut visited_positions = BTreeSet::from([guard_pos]);

    while let Some(pos) = get_next_position(&map, &guard_pos, &current_delta) {
        match map.tiles[pos.y as usize][pos.x as usize] {
            Tile::Obstruction => {
                current_delta = deltas_iter.next().unwrap();
            }
            Tile::Space => {
                guard_pos = pos;
                visited_positions.insert(pos);
            }
        }
    }

    Ok(visited_positions.len() as u32)
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

        assert_eq!(result, 0);
    }
}
