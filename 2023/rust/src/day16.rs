use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn step(&self, dir: &Direction) -> Option<Self> {
        match *dir {
            Direction::Up => Some(Position::new(self.x, self.y.checked_sub(1)?)),
            Direction::Left => Some(Position::new(self.x.checked_sub(1)?, self.y)),
            Direction::Down => Some(Position::new(self.x, self.y + 1)),
            Direction::Right => Some(Position::new(self.x + 1, self.y)),
        }
    }

    fn step_beam(&self, dir: &Direction) -> Option<Beam> {
        self.step(dir).map(|p| (p, *dir))
    }
}

#[derive(Debug)]
struct Map {
    inner: Vec<Vec<u8>>,
}

impl Map {
    fn parse<'a, I, S>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut map = Map { inner: Vec::new() };
        for line in lines.into_iter().map(|l| l.as_ref().bytes().collect()) {
            map.inner.push(line);
        }
        map
    }

    fn at(&self, pos: &Position) -> Option<u8> {
        self.inner.get(pos.y)?.get(pos.x).copied()
    }
}

enum ProcessBeamOutput {
    Forget,
    Moved(Beam),
    Split(Option<Beam>, Option<Beam>),
}

fn process_beam(
    map: &Map,
    snapshots: &mut HashSet<Beam>,
    pos: &Position,
    facing: &Direction,
) -> ProcessBeamOutput {
    let Some(tile) = map.at(pos) else {
        return ProcessBeamOutput::Forget;
    };

    if !snapshots.insert((*pos, *facing)) {
        return ProcessBeamOutput::Forget;
    }

    match tile {
        b'.' => {
            if let Some(beam) = pos.step_beam(facing) {
                ProcessBeamOutput::Moved(beam)
            } else {
                ProcessBeamOutput::Forget
            }
        }
        b'\\' => {
            let next_dir = match *facing {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
            };
            if let Some(next_beam) = pos.step_beam(&next_dir) {
                ProcessBeamOutput::Moved(next_beam)
            } else {
                ProcessBeamOutput::Forget
            }
        }
        b'/' => {
            let next_dir = match *facing {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
            };
            if let Some(next_beam) = pos.step_beam(&next_dir) {
                ProcessBeamOutput::Moved(next_beam)
            } else {
                ProcessBeamOutput::Forget
            }
        }
        b'-' => {
            if *facing == Direction::Up || *facing == Direction::Down {
                let split1 = pos.step_beam(&Direction::Left);
                let split2 = pos.step_beam(&Direction::Right);
                ProcessBeamOutput::Split(split1, split2)
            } else {
                let next_beam = pos.step_beam(facing);
                if let Some(next_beam) = next_beam {
                    ProcessBeamOutput::Moved(next_beam)
                } else {
                    ProcessBeamOutput::Forget
                }
            }
        }
        b'|' => {
            if *facing == Direction::Left || *facing == Direction::Right {
                let split1 = pos.step_beam(&Direction::Down);
                let split2 = pos.step_beam(&Direction::Up);
                ProcessBeamOutput::Split(split1, split2)
            } else {
                let next_beam = pos.step_beam(facing);
                if let Some(next_beam) = next_beam {
                    ProcessBeamOutput::Moved(next_beam)
                } else {
                    ProcessBeamOutput::Forget
                }
            }
        }
        unknown => panic!("Unknown character in input: '{}'", unknown),
    }
}

type Beam = (Position, Direction);

fn solve_from(map: &Map, beam: Beam) -> u32 {
    let mut beam_snapshots: HashSet<Beam> = HashSet::new();
    let mut beams: Vec<Beam> = vec![beam];

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = Vec::new();
        let mut beams_to_remove: Vec<usize> = Vec::new();

        for (i, (beam_pos, beam_dir)) in beams.iter_mut().enumerate().rev() {
            let res = process_beam(map, &mut beam_snapshots, beam_pos, beam_dir);
            match res {
                ProcessBeamOutput::Forget => {
                    beams_to_remove.push(i);
                }
                ProcessBeamOutput::Moved((next_beam_pos, next_beam_dir)) => {
                    *beam_pos = next_beam_pos;
                    *beam_dir = next_beam_dir;
                }
                ProcessBeamOutput::Split(s1, s2) => {
                    if let Some(s1) = s1 {
                        new_beams.push(s1);
                    }
                    if let Some(s2) = s2 {
                        new_beams.push(s2);
                    }
                }
            }
        }

        for i in beams_to_remove.drain(..) {
            beams.remove(i);
        }

        beams.append(&mut new_beams);
    }

    let unique_positions: HashSet<_> = beam_snapshots.iter().map(|(p, _)| p).collect();
    unique_positions.len() as u32
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = Map::parse(lines);
    Ok(solve_from(&map, (Position::new(0, 0), Direction::Right)))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = Map::parse(lines);
    let width = map.inner[0].len();
    let height = map.inner.len();
    let mut starting_beams = vec![];

    for x in 0..width {
        starting_beams.push((Position::new(x, 0), Direction::Down));
        starting_beams.push((Position::new(x, height - 1), Direction::Up));
    }

    for y in 0..height {
        starting_beams.push((Position::new(0, y), Direction::Right));
        starting_beams.push((Position::new(width - 1, y), Direction::Left));
    }

    Ok(starting_beams
        .iter()
        .map(|b| solve_from(&map, *b))
        .max()
        .expect("There is a maximum value"))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        ".|...\\....",
        "|.-.\\.....",
        ".....|-...",
        "........|.",
        "..........",
        ".........\\",
        "..../.\\\\..",
        ".-.-/..|..",
        ".|....-|.\\",
        "..//.|....",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 46);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 51);
    }
}
