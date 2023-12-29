use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn from_byte(b: u8) -> anyhow::Result<Self> {
        match b {
            b'U' => Ok(Self::Up),
            b'L' => Ok(Self::Left),
            b'D' => Ok(Self::Down),
            b'R' => Ok(Self::Right),
            unknown => Err(anyhow!("Tried to parse unknown direction '{}'", unknown)),
        }
    }
}

struct Instruction<'a> {
    direction: Direction,
    distance: u32,
    color: &'a str,
}

impl<'a> Instruction<'a> {
    fn parse(line: &'a str) -> anyhow::Result<Self> {
        let mut iter = line.split_ascii_whitespace();
        let dir = Direction::from_byte(
            iter.next()
                .ok_or(anyhow!("Could not parse direction string"))?
                .bytes()
                .next()
                .ok_or(anyhow!("Could not get first byte of dir_str"))?,
        )?;
        let dist = iter
            .next()
            .ok_or(anyhow!("Could not parse distance string"))?
            .parse::<u32>()?;
        let color_str = iter.next().ok_or(anyhow!("Could not parse color string"))?;
        let color = &color_str[1..color_str.len() - 1];

        Ok(Instruction {
            direction: dir,
            distance: dist,
            color,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Position {
    fn mov(&self, ins: &Instruction) -> Position {
        let d = ins.distance as i32;
        match ins.direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - d,
            },
            Direction::Left => Position {
                x: self.x - d,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + d,
            },
            Direction::Right => Position {
                x: self.x + d,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    p1: Position,
    p2: Position,
}

impl Edge {
    fn new(p1: Position, p2: Position) -> Self {
        Self { p1, p2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Chunk {
    p1: Position,
    p2: Position,
}

impl Chunk {
    fn new(p1: Position, p2: Position) -> Self {
        Self { p1, p2 }
    }

    fn get_width(&self) -> u32 {
        self.p1.x.abs_diff(self.p2.x)
    }

    fn get_height(&self) -> u32 {
        self.p1.y.abs_diff(self.p2.y)
    }

    fn get_area(&self) -> u32 {
        self.get_width() * self.get_height()
    }

    fn halve(&self) -> Option<(Chunk, Chunk)> {
        if self.get_area() == 1 {
            return None;
        }

        let split_horizontally = self.get_height() >= self.get_width();

        let splits = if split_horizontally {
            let halved_height = (self.get_height() / 2) as i32;
            let a1 = Chunk::new(self.p1, Position::new(self.p2.x, self.p1.y - halved_height));
            let a2 = Chunk::new(
                Position::new(
                    self.p1.x,
                    self.p2.y + self.get_height() as i32 - halved_height,
                ),
                self.p2,
            );
            (a1, a2)
        } else {
            let halved_width = (self.get_width() / 2) as i32;
            let a1 = Chunk::new(self.p1, Position::new(self.p2.x - halved_width, self.p2.y));
            let a2 = Chunk::new(
                Position::new(
                    self.p1.x + self.get_width() as i32 - halved_width,
                    self.p1.y,
                ),
                self.p2,
            );
            (a1, a2)
        };
        Some(splits)
    }

    fn get_bounding_edges(&self) -> [Edge; 4] {
        let p3 = Position::new(self.p1.x, self.p2.y);
        let p4 = Position::new(self.p2.x, self.p1.y);

        [
            Edge::new(self.p1, p3),
            Edge::new(self.p1, p4),
            Edge::new(self.p2, p3),
            Edge::new(self.p2, p4),
        ]
    }

    fn count_intersections(&self, edges: &Vec<Edge>) -> u32 {
        // TODO:
        0
    }

    fn is_inside(&self, edges: &Vec<Edge>, original_chunk: &Chunk) -> bool {
        // TODO:
        false
    }
}

fn solve_part1(edges: &Vec<Edge>, chunk: &Chunk, original_chunk: &Chunk) -> u32 {
    if chunk.count_intersections(edges) == 0 && chunk.is_inside(edges, original_chunk) {
        chunk.get_area()
    } else {
        let Some((h1, h2)) = chunk.halve() else {
            return if chunk.is_inside(edges, original_chunk) {
                1
            } else {
                0
            };
        };
        solve_part1(edges, &h1, original_chunk) + solve_part1(edges, &h2, original_chunk)
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut edges = Vec::new();
    let mut previous_pos = Position::new(0, 0);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let ins = Instruction::parse(line)?;
        let current_pos = previous_pos.mov(&ins);

        if min_x > current_pos.x {
            min_x = current_pos.x;
        }
        if max_x < current_pos.x {
            max_x = current_pos.x;
        }
        if min_y > current_pos.y {
            min_y = current_pos.y;
        }
        if max_y < current_pos.y {
            max_y = current_pos.y;
        }

        edges.push(Edge::new(previous_pos, current_pos));
        previous_pos = current_pos;
    }

    let area = Chunk::new(Position::new(min_x, min_y), Position::new(max_x, max_y));

    Ok(solve_part1(&edges, &area, &area))
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
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 62);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn chunk_works() {
        let chunk = Chunk {
            p1: Position::new(-2, 18),
            p2: Position::new(10, 5),
        };
        let (c1, c2) = chunk.halve().expect("halve() to return a Some");
        assert_eq!(c1, Chunk::new(Position::new(-2, 18), Position::new(10, 12)));
        assert_eq!(c2, Chunk::new(Position::new(-2, 12), Position::new(10, 5)));

        let area = chunk.get_area();
        assert_eq!(area, c1.get_area() + c2.get_area());

        let chunk2 = Chunk {
            p1: Position::new(0, 0),
            p2: Position::new(7, 5),
        };
        let (c3, c4) = chunk2.halve().expect("halve() to return a Some");
        assert_eq!(c3, Chunk::new(Position::new(0, 0), Position::new(4, 5)));
        assert_eq!(c4, Chunk::new(Position::new(4, 0), Position::new(7, 5)));

        let area2 = chunk2.get_area();
        assert_eq!(area2, c3.get_area() + c4.get_area());

        let chunk3 = Chunk {
            p1: Position::new(3, 4),
            p2: Position::new(2, 5),
        };
        assert_eq!(chunk3.get_area(), 1);
        assert_eq!(chunk3.halve(), None);
    }
}
