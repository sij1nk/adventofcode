use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct V2 {
    x: i32,
    y: i32,
}

impl V2 {
    fn add(&self, other: &V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn is_in_bounds(&self, map_width: i32, map_height: i32) -> bool {
        self.x >= 0 && self.x < map_width && self.y >= 0 && self.y < map_height
    }
}

type Antennas = BTreeMap<char, Vec<V2>>;

struct Map {
    antennas: Antennas,
    width: i32,
    height: i32,
}

fn parse<'a, I, S>(lines: I) -> Map
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut antennas: BTreeMap<char, Vec<V2>> = BTreeMap::new();

    let mut width = None;
    let mut height = 0;

    for (y, line) in lines.into_iter().map(|s| s.as_ref()).enumerate() {
        height += 1;
        width.get_or_insert(line.len() as i32);

        for (x, c) in line.bytes().map(char::from).enumerate() {
            match c {
                '.' => continue,
                _ => {
                    let pos = V2 {
                        x: x as i32,
                        y: y as i32,
                    };
                    if let Some(cs) = antennas.get_mut(&c) {
                        cs.push(pos);
                    } else {
                        antennas.insert(c, vec![pos]);
                    }
                }
            }
        }
    }

    Map {
        antennas,
        width: width.unwrap(),
        height,
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = parse(lines);

    let mut antinode_positions = BTreeSet::<V2>::new();

    for ants in map.antennas.values() {
        for combo in ants.iter().combinations(2) {
            let a1 = combo[0];
            let a2 = combo[1];

            let diff = a1.sub(a2);

            let p1 = a1.add(&diff);
            let p2 = a2.sub(&diff);

            if p1.is_in_bounds(map.width, map.height) {
                antinode_positions.insert(p1);
            }

            if p2.is_in_bounds(map.width, map.height) {
                antinode_positions.insert(p2);
            }
        }
    }

    Ok(antinode_positions.len() as u32)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = parse(lines);

    let mut antinode_positions = BTreeSet::<V2>::new();

    antinode_positions.extend(map.antennas.values().flatten());

    for ants in map.antennas.values() {
        for combo in ants.iter().combinations(2) {
            let a1 = combo[0];
            let a2 = combo[1];

            let diff = a1.sub(a2);

            let mut current = *a1;
            loop {
                let p = current.add(&diff);

                if !p.is_in_bounds(map.width, map.height) {
                    break;
                }

                antinode_positions.insert(p);
                current = p;
            }

            current = *a2;
            loop {
                let p = current.sub(&diff);

                if !p.is_in_bounds(map.width, map.height) {
                    break;
                }

                antinode_positions.insert(p);
                current = p;
            }
        }
    }

    Ok(antinode_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "............",
        "........0...",
        ".....0......",
        ".......0....",
        "....0.......",
        "......A.....",
        "............",
        "............",
        "........A...",
        ".........A..",
        "............",
        "............",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 14);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 34);
    }
}
