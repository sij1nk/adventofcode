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

#[derive(Debug)]
struct EdgePositionsIter {
    current_x: i32,
    current_y: i32,
    step_x: i32,
    step_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Iterator for EdgePositionsIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x > self.max_x || self.current_y > self.max_y {
            None
        } else {
            let p = Position::new(self.current_x, self.current_y);
            self.current_x += self.step_x;
            self.current_y += self.step_y;
            Some(p)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EdgeRelationshipType {
    Separate,
    Intersecting,
    Covering,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    p1: Position,
    p2: Position,
}

impl Edge {
    // TODO: should only accept edges which are parallel to the x or y axis
    // TODO: should accept 2 tuples of (i32, i32) too (try_from)
    fn new(p1: Position, p2: Position) -> Self {
        Self { p1, p2 }
    }

    fn iter_positions(&self) -> EdgePositionsIter {
        let diff_x = self.p1.x.abs_diff(self.p2.x) as i32;
        let diff_y = self.p1.y.abs_diff(self.p2.y) as i32;
        let min_x = self.p1.x.min(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);

        EdgePositionsIter {
            current_x: min_x,
            current_y: min_y,
            step_x: if diff_x == 0 { 0 } else { 1 },
            step_y: if diff_y == 0 { 0 } else { 1 },
            max_x: min_x + diff_x,
            max_y: min_y + diff_y,
        }
    }

    fn is_parallel_to(&self, other: &Edge) -> bool {
        let diff_x = self.p1.x.abs_diff(self.p2.x) as i32;
        let diff_y = self.p1.y.abs_diff(self.p2.y) as i32;
        let other_diff_x = other.p1.x.abs_diff(other.p2.x);
        let other_diff_y = other.p1.y.abs_diff(other.p2.y);

        (diff_x == 0 && other_diff_x == 0) || (diff_y == 0 && other_diff_y == 0)
    }

    fn get_relationship_to(&self, other: &Edge) -> EdgeRelationshipType {
        let self_positions_iter = self.iter_positions();
        let other_positions: Vec<_> = other.iter_positions().collect();

        let intersecting_positions = self_positions_iter.filter(|&p| other_positions.contains(&p));

        // If there are more than 1 intersecting positions, it means that one of the edges
        // (partially) covers the other, and they are parallel. We don't consider such edges as
        // intersecting
        match intersecting_positions.count() {
            0 => EdgeRelationshipType::Separate,
            1 => EdgeRelationshipType::Intersecting,
            _ => EdgeRelationshipType::Covering,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Chunk {
    min_corner: Position,
    max_corner: Position,
}

impl Chunk {
    fn new(p1: Position, p2: Position) -> Self {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        Self {
            min_corner: Position::new(min_x, min_y),
            max_corner: Position::new(max_x, max_y),
        }
    }

    fn get_width(&self) -> u32 {
        self.min_corner.x.abs_diff(self.max_corner.x)
    }

    fn get_height(&self) -> u32 {
        self.min_corner.y.abs_diff(self.max_corner.y)
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
            let a1 = Chunk::new(
                self.min_corner,
                Position::new(self.max_corner.x, self.max_corner.y - halved_height),
            );
            let a2 = Chunk::new(
                Position::new(self.min_corner.x, self.max_corner.y - halved_height),
                self.max_corner,
            );
            (a1, a2)
        } else {
            let halved_width = (self.get_width() / 2) as i32;
            let a1 = Chunk::new(
                self.min_corner,
                Position::new(self.min_corner.x + halved_width, self.max_corner.y),
            );
            let a2 = Chunk::new(
                Position::new(self.min_corner.x + halved_width, self.min_corner.y),
                self.max_corner,
            );
            (a1, a2)
        };
        Some(splits)
    }

    fn get_bounding_edges(&self) -> [Edge; 4] {
        let p3 = Position::new(self.min_corner.x, self.max_corner.y);
        let p4 = Position::new(self.max_corner.x, self.min_corner.y);

        [
            Edge::new(self.min_corner, p3),
            Edge::new(self.min_corner, p4),
            Edge::new(self.max_corner, p3),
            Edge::new(self.max_corner, p4),
        ]
    }

    fn is_pierced_by_edges(&self, edges: &Vec<Edge>) -> bool {
        if self.max_corner.x - self.min_corner.x <= 1 && self.max_corner.y - self.min_corner.y <= 1
        {
            return false;
        }

        let bounding_edges = self.get_bounding_edges();

        for edge in edges.iter() {
            let bounding_edge_relationships = bounding_edges
                .iter()
                .map(|be| be.get_relationship_to(edge))
                .collect::<Vec<_>>();
            let is_bounding_edge =
                bounding_edge_relationships.contains(&EdgeRelationshipType::Covering);
            let intersected_count = bounding_edge_relationships
                .iter()
                .filter(|&rel| *rel == EdgeRelationshipType::Intersecting)
                .count();
            if intersected_count == 2 && !is_bounding_edge {
                return true;
            }
            for pos in edge.iter_positions() {
                if pos.x > self.min_corner.x
                    && pos.x < self.max_corner.x
                    && pos.y > self.min_corner.y
                    && pos.y < self.max_corner.y
                {
                    return true;
                }
            }
        }

        false
    }

    // TODO: use floats
    fn is_inside(&self, edges: &Vec<Edge>, main_chunk: &Chunk) -> bool {
        let p_start = self.min_corner;
        let p_end = {
            let y = p_start.y;
            // let x = if p_start.x.abs_diff(main_chunk.min_corner.x)
            //     < p_start.x.abs_diff(main_chunk.max_corner.x)
            // {
            //     main_chunk.min_corner.x - 1
            // } else {
            //     main_chunk.max_corner.x + 1
            // };
            let x = main_chunk.min_corner.x - 1;
            Position::new(x, y)
        };
        let edge = Edge::new(p_start, p_end);

        let intersections = edges.iter().filter(|&e| {
            e.get_relationship_to(&edge) == EdgeRelationshipType::Intersecting
                && !e.is_parallel_to(&edge)
        });

        let inside_cnt = intersections.count();
        inside_cnt % 2 == 1
        // intersections.count() % 2 == 1
    }
}

fn solve_part1(edges: &Vec<Edge>, chunk: &Chunk, main_chunk: &Chunk) -> u32 {
    if !chunk.is_pierced_by_edges(edges) && chunk.is_inside(edges, main_chunk) {
        let area = chunk.get_area();
        println!("+ {}, {:?}", area, chunk);
        area
    } else {
        let Some((h1, h2)) = chunk.halve() else {
            return if chunk.is_inside(edges, main_chunk) {
                println!("+ 1, {:?}", chunk);
                1
            } else {
                0
            };
        };
        solve_part1(edges, &h1, main_chunk) + solve_part1(edges, &h2, main_chunk)
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
    fn part1_test18() {
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
        let chunk = Chunk::new(Position::new(-2, 18), Position::new(10, 5));
        let (c1, c2) = chunk.halve().expect("halve() to return a Some");
        assert_eq!(c1, Chunk::new(Position::new(-2, 5), Position::new(10, 12)));
        assert_eq!(c2, Chunk::new(Position::new(-2, 12), Position::new(10, 18)));

        let area = chunk.get_area();
        assert_eq!(area, c1.get_area() + c2.get_area());

        let chunk2 = Chunk::new(Position::new(0, 0), Position::new(7, 5));
        let (c3, c4) = chunk2.halve().expect("halve() to return a Some");
        assert_eq!(c3, Chunk::new(Position::new(0, 0), Position::new(3, 5)));
        assert_eq!(c4, Chunk::new(Position::new(3, 0), Position::new(7, 5)));

        let area2 = chunk2.get_area();
        assert_eq!(area2, c3.get_area() + c4.get_area());

        let chunk3 = Chunk::new(Position::new(3, 4), Position::new(2, 5));
        assert_eq!(chunk3.get_area(), 1);
        assert_eq!(chunk3.halve(), None);
    }

    #[test]
    fn edge_iter_positions_works() {
        let edge = Edge::new(Position::new(3, 5), Position::new(3, -1));
        let edge_positions: Vec<Position> = edge.iter_positions().collect();
        assert_eq!(
            edge_positions,
            vec![
                Position::new(3, -1),
                Position::new(3, 0),
                Position::new(3, 1),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 4),
                Position::new(3, 5)
            ]
        );
    }

    #[test]
    fn edge_intersects_works() {
        let e1 = Edge::new(Position::new(3, 5), Position::new(3, -1));
        let e2 = Edge::new(Position::new(0, 0), Position::new(6, 0));
        let e3 = Edge::new(Position::new(2, 5), Position::new(2, -1));

        assert_eq!(
            e1.get_relationship_to(&e2),
            EdgeRelationshipType::Intersecting
        );
        assert_eq!(e1.get_relationship_to(&e3), EdgeRelationshipType::Separate);
    }
}
