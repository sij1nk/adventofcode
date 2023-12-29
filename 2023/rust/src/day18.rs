use std::collections::HashSet;

use anyhow::anyhow;

trait FloatExt {
    fn fround(self, fractional_digits: u32) -> Self;
    fn eq(self, other: Self) -> bool;
}

impl FloatExt for f64 {
    fn fround(self, fractional_digits: u32) -> Self {
        let n = 10u32.pow(fractional_digits) as Self;
        (self * n).round() / n
    }

    fn eq(self, other: Self) -> bool {
        let epsilon = 0.0005;
        (self - other).abs() < epsilon
    }
}

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
    x: f64,
    y: f64,
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x as f64, y as f64)
    }
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl Position {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn mov(&self, ins: &Instruction) -> Position {
        let d = ins.distance as f64;
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
    current_x: f64,
    current_y: f64,
    step_x: f64,
    step_y: f64,
    max_x: f64,
    max_y: f64,
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

impl From<[(i32, i32); 2]> for Edge {
    fn from(positions: [(i32, i32); 2]) -> Self {
        Self::new(Position::from(positions[0]), Position::from(positions[1]))
    }
}

impl From<[(f64, f64); 2]> for Edge {
    fn from(positions: [(f64, f64); 2]) -> Self {
        Self::new(Position::from(positions[0]), Position::from(positions[1]))
    }
}

impl Edge {
    // TODO: should only accept edges which are parallel to the x or y axis
    // TODO: should accept 2 tuples of (f64, f64) too (try_from)
    fn new(p1: Position, p2: Position) -> Self {
        Self { p1, p2 }
    }

    fn iter_positions(&self) -> EdgePositionsIter {
        let diff_x = (self.p1.x - self.p2.x).abs().fround(1);
        let diff_y = (self.p1.y - self.p2.y).abs().fround(1);
        let min_x = self.p1.x.min(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);

        EdgePositionsIter {
            current_x: min_x,
            current_y: min_y,
            step_x: if diff_x.eq(0.0) { 0.0 } else { 1.0 },
            step_y: if diff_y.eq(0.0) { 0.0 } else { 1.0 },
            max_x: min_x + diff_x,
            max_y: min_y + diff_y,
        }
    }

    fn is_parallel_to(&self, other: &Edge) -> bool {
        let diff_x = (self.p1.x - self.p2.x).abs().fround(1);
        let diff_y = (self.p1.y - self.p2.y).abs().fround(1);
        let other_diff_x = (other.p1.x - other.p2.x).abs().fround(1);
        let other_diff_y = (other.p1.y - other.p2.y).abs().fround(1);

        (diff_x.eq(0.0) && other_diff_x.eq(0.0)) || (diff_y.eq(0.0) && other_diff_y.eq(0.0))
    }

    /// Returns the intersection point of two lines, represented by self and other, if it exists.
    /// Can produce an intersection point not contained in either edges.
    fn get_line_intersection(&self, other: &Edge) -> Option<Position> {
        let x1 = self.p1.x;
        let x2 = self.p2.x;
        let x3 = other.p1.x;
        let x4 = other.p2.x;
        let y1 = self.p1.y;
        let y2 = self.p2.y;
        let y3 = other.p1.y;
        let y4 = other.p2.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator.eq(0.0) {
            return None;
        }

        let num_x = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let num_y = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        let px = (num_x / denominator).fround(1);
        let py = (num_y / denominator).fround(1);
        Some(Position::new(px, py))
    }

    /// Returns the intersection point of two segments (edges), represented by self and other, if it exists.
    /// NOTE: here we don't actually care about the exact intersection point, we only care whether
    /// it exists
    fn get_edge_intersection(&self, other: &Edge) -> Option<Position> {
        let x1 = self.p1.x;
        let x2 = self.p2.x;
        let x3 = other.p1.x;
        let x4 = other.p2.x;
        let y1 = self.p1.y;
        let y2 = self.p2.y;
        let y3 = other.p1.y;
        let y4 = other.p2.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let num_t = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let num_u = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);

        let t = (num_t / denominator).fround(1);
        let u = (num_u / denominator).fround(1);

        if t < 0.0 || 1.0 < t || u < 0.0 || 1.0 < u {
            return None;
        }

        let px = x1 + t * (x2 - x1);
        let py = y1 + t * (y2 - y1);
        Some(Position::new(px, py))
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

impl From<[(i32, i32); 2]> for Chunk {
    fn from(positions: [(i32, i32); 2]) -> Self {
        Self::new(Position::from(positions[0]), Position::from(positions[1]))
    }
}

impl From<[(f64, f64); 2]> for Chunk {
    fn from(positions: [(f64, f64); 2]) -> Self {
        Self::new(Position::from(positions[0]), Position::from(positions[1]))
    }
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

    fn get_width(&self) -> f64 {
        self.max_corner.x - self.min_corner.x
    }

    fn get_height(&self) -> f64 {
        self.max_corner.y - self.min_corner.y
    }

    fn get_area(&self) -> f64 {
        self.get_width() * self.get_height()
    }

    fn halve(&self) -> Option<(Chunk, Chunk)> {
        if self.get_area().eq(1.0) {
            return None;
        }

        let split_horizontally = self.get_height() >= self.get_width();

        let splits = if split_horizontally {
            let halved_height = (self.get_height() / 2.0).floor();
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
            let halved_width = (self.get_width() / 2.0).floor();
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

    fn is_pierced_by_edges(&self, edges: &[Edge]) -> bool {
        if self.max_corner.x - self.min_corner.x <= 1.0
            && self.max_corner.y - self.min_corner.y <= 1.0
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

    fn is_inside(&self, edges: &[Edge], main_chunk: &Chunk) -> bool {
        let p_start = {
            let x = self.min_corner.x;
            let y = self.min_corner.y + 0.5;
            Position::new(x, y)
        };
        let p_end = {
            let x = main_chunk.min_corner.x - 1.0;
            let y = p_start.y;
            Position::new(x, y)
        };
        let probe = Edge::new(p_start, p_end);

        let intersections = edges
            .iter()
            .filter_map(|&e| e.get_edge_intersection(&probe));

        intersections.count() % 2 == 1
    }

    fn iter_positions(&self) -> ChunkPositionsIter {
        ChunkPositionsIter {
            start_x: self.min_corner.x,
            current_x: self.min_corner.x,
            current_y: self.min_corner.y,
            max_x: self.max_corner.x,
            max_y: self.max_corner.y,
        }
    }
}

#[derive(Debug)]
struct ChunkPositionsIter {
    start_x: f64,
    current_x: f64,
    current_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Iterator for ChunkPositionsIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x > self.max_x {
            self.current_x = self.start_x;
            self.current_y += 1.0;
        }

        if self.current_y > self.max_y {
            None
        } else {
            let p = Position::new(self.current_x.round(), self.current_y.round());
            self.current_x += 1.0;
            Some(p)
        }
    }
}

fn solve_part1(
    edges: &Vec<Edge>,
    chunk: &Chunk,
    main_chunk: &Chunk,
    contained_positions: &mut HashSet<(i32, i32)>,
) -> () {
    println!("Solve part1: {:?}", chunk);
    if !chunk.is_pierced_by_edges(edges) && chunk.is_inside(edges, main_chunk) {
        for pos in chunk.iter_positions().map(|p| (p.x as i32, p.y as i32)) {
            contained_positions.insert(pos);
        }
        return;
    } else {
        let Some((h1, h2)) = chunk.halve() else {
            if chunk.is_inside(edges, main_chunk) {
                for pos in chunk.iter_positions().map(|p| (p.x as i32, p.y as i32)) {
                    contained_positions.insert(pos);
                }
            }
            return;
        };
        solve_part1(edges, &h1, main_chunk, contained_positions);
        solve_part1(edges, &h2, main_chunk, contained_positions);
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut edges = Vec::new();
    let mut previous_pos = Position::new(0.0, 0.0);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0.0, 0.0, 0.0, 0.0);

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

    let area = Chunk::from([(min_x, min_y), (max_x, max_y)]);
    println!("Area: {:?}", area);
    let mut contained_positions: HashSet<(i32, i32)> = HashSet::new();
    solve_part1(&edges, &area, &area, &mut contained_positions);

    Ok(contained_positions.len() as u32)
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
        let chunk = Chunk::from([(-2.0, 18.0), (10.0, 5.0)]);
        let (c1, c2) = chunk.halve().expect("halve() to return a Some");
        assert_eq!(c1, Chunk::from([(-2.0, 5.0), (10.0, 12.0)]));
        assert_eq!(c2, Chunk::from([(-2.0, 12.0), (10.0, 18.0)]));

        let area = chunk.get_area();
        assert_eq!(area, c1.get_area() + c2.get_area());

        let chunk2 = Chunk::from([(0.0, 0.0), (7.0, 5.0)]);
        let (c3, c4) = chunk2.halve().expect("halve() to return a Some");
        assert_eq!(c3, Chunk::from([(0, 0), (3, 5)]));
        assert_eq!(c4, Chunk::from([(3, 0), (7, 5)]));

        let area2 = chunk2.get_area();
        assert_eq!(area2, c3.get_area() + c4.get_area());

        let chunk3 = Chunk::from([(3, 4), (2, 5)]);
        assert_eq!(chunk3.get_area(), 1.0);
        assert_eq!(chunk3.halve(), None);
    }

    #[test]
    fn chunk_iter_works() {
        let chunk = Chunk::from([(-2, 18), (10, 5)]);
        let iter = chunk.iter_positions();
        assert_eq!(iter.count(), 13 * 14);
    }

    #[test]
    fn edge_iter_positions_works() {
        let edge = Edge::from([(3, 5), (3, -1)]);
        let edge_positions: Vec<Position> = edge.iter_positions().collect();
        assert_eq!(
            edge_positions,
            vec![
                (3, -1).into(),
                (3, -0).into(),
                (3, 1).into(),
                (3, 2).into(),
                (3, 3).into(),
                (3, 4).into(),
                (3, 5).into(),
            ]
        );
    }

    #[test]
    fn edge_intersects_works() {
        let e1 = Edge::from([(3, 5), (3, -1)]);
        let e2 = Edge::from([(0, 0), (6, 0)]);
        let e3 = Edge::from([(2, 5), (2, -1)]);

        assert_eq!(
            e1.get_relationship_to(&e2),
            EdgeRelationshipType::Intersecting
        );
        assert_eq!(e1.get_relationship_to(&e3), EdgeRelationshipType::Separate);
    }

    #[test]
    fn edge_get_line_intersection_works() {
        let e1 = Edge::from([(1, 5), (5, 5)]);
        let e2 = Edge::from([(5, 2), (5, 6)]);
        let e3 = Edge::from([(6, 2), (6, 6)]);
        let e4 = Edge::from([(1, 3), (3, 3)]);

        assert_eq!(e1.get_line_intersection(&e2), Some((5, 5).into()));
        assert_eq!(e1.get_line_intersection(&e3), Some((6, 5).into()));
        assert_eq!(e1.get_line_intersection(&e4), None);
    }

    #[test]
    fn edge_get_edge_intersection_works() {
        let e1 = Edge::from([(1, 5), (5, 5)]);
        let e2 = Edge::from([(5, 2), (5, 6)]);
        let e3 = Edge::from([(6, 2), (6, 6)]);
        let e4 = Edge::from([(1, 3), (3, 3)]);

        assert_eq!(e1.get_edge_intersection(&e2), Some((5, 5).into()));
        assert_eq!(e1.get_edge_intersection(&e3), None);
        assert_eq!(e1.get_edge_intersection(&e4), None);
    }

    #[test]
    fn fround_works() {
        assert_eq!(2.45.fround(2), 2.45);
        assert_eq!(2.45.fround(1), 2.5);
        assert_eq!(2.45.fround(0), 2.0);
        assert_eq!(2.216.fround(2), 2.22);
        assert_eq!(2.216.fround(1), 2.2);
        assert_eq!(2.216.fround(0), 2.0);
    }
}
