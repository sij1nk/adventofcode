use colored::Colorize;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    iter::{Skip, Take},
    slice::Iter,
};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

const DIRECTION_PRIORITY: &[Direction] = &[
    Direction::Down,
    Direction::Right,
    Direction::Up,
    Direction::Left,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn towards(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => Some(Self::new(self.x, self.y.checked_sub(1)?)),
            Direction::Left => Some(Self::new(self.x.checked_sub(1)?, self.y)),
            Direction::Down => Some(Self::new(self.x, self.y + 1)),
            Direction::Right => Some(Self::new(self.x + 1, self.y)),
        }
    }

    fn distance(&self, other: Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse<'a, I, S>(lines: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut data = Vec::new();

        let mut lines = lines.into_iter().map(|l| l.as_ref());
        let first_line = lines.next().ok_or(anyhow!("Input should not be empty"))?;
        let width = first_line.len();

        let to_chars = |l: &'a str| l.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8));

        data.extend(to_chars(first_line));

        for line in lines.map(to_chars) {
            data.extend(line);
        }

        let height = data.len() / width;

        Ok(Map {
            data,
            width,
            height,
        })
    }

    fn at(&self, pos: Position) -> Option<u8> {
        if pos.x >= self.width || pos.y >= self.height {
            None
        } else {
            let i = pos.y * self.width + pos.x;
            self.data.get(i).copied()
        }
    }

    fn rows_iter(&self) -> MapRowsIter {
        MapRowsIter {
            data: &self.data,
            width: self.width,
            start: 0,
        }
    }
}

struct MapRowsIter<'a> {
    data: &'a [u8],
    width: usize,
    start: usize,
}

impl<'a> Iterator for MapRowsIter<'a> {
    type Item = Take<Skip<Iter<'a, u8>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.data.len() / self.width {
            return None;
        }
        let iter = self
            .data
            .iter()
            .skip(self.start * self.width)
            .take(self.width);
        self.start += 1;
        Some(iter)
    }
}

fn heuristic(map: &Map, pos: Position) -> usize {
    pos.distance(Position::new(map.width - 1, map.height - 1))
}

#[derive(Clone)]
struct SearchNode {
    from: Option<Box<SearchNode>>,
    pos: Position,
    g: usize,
    h: usize,
    facing: Direction,
    straightness: usize,
}

impl Debug for SearchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchNode")
            .field("pos", &self.pos)
            .field("g", &self.g)
            .field("h", &self.h)
            .field("facing", &self.facing)
            .field("straightness", &self.straightness)
            .finish()
    }
}

impl SearchNode {
    fn new(
        map: &Map,
        from: Option<Box<SearchNode>>,
        pos: Position,
        g: usize,
        facing: Direction,
        straightness: usize,
    ) -> Self {
        Self {
            from,
            pos,
            g,
            h: heuristic(map, pos),
            facing,
            straightness,
        }
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        let f = self.g + self.h;
        let other_f = other.g + other.h;
        f == other_f
    }
}

impl Eq for SearchNode {}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let f = self.g + self.h;
        let other_f = other.g + other.h;
        other_f.cmp(&f)
    }
}

fn print_final_path(map: &Map, node: &SearchNode) {
    let mut touched_positions = HashSet::new();

    touched_positions.insert(node.pos);
    let mut node = Box::new(node.clone());
    while let Some(ref previous) = node.from {
        println!("Prev: {:?}", previous.pos);
        touched_positions.insert(previous.pos);
        node = previous.clone();
    }

    for (y, row) in map.rows_iter().enumerate() {
        for (x, n) in row.enumerate() {
            let pos = Position::new(x, y);
            if touched_positions.contains(&pos) {
                print!("{}", n.to_string().white().on_red());
            } else {
                print!("{n}");
            }
        }
        println!();
    }
}

fn solve(map: &Map, start: Position) -> anyhow::Result<usize> {
    let goal = Position::new(map.width - 1, map.height - 1);
    let mut touched_positions: HashSet<(Position, Direction, usize)> = HashSet::new();

    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();
    heap.push(SearchNode::new(map, None, start, 0, Direction::Right, 0));
    touched_positions.insert((start, Direction::Right, 0));

    loop {
        let Some(node) = heap.pop() else {
            return Err(anyhow!("A* ran out of search nodes"));
        };

        if node.pos == goal {
            print_final_path(map, &node);
            return Ok(node.g);
        }

        for dir in DIRECTION_PRIORITY {
            if node.facing.opposite() == *dir {
                // can't turn 180 degrees
                continue;
            }
            let same_direction = *dir == node.facing;
            let new_straightness = if same_direction {
                node.straightness + 1
            } else {
                1
            };
            let Some(neighbor_position) = node.pos.towards(*dir) else {
                // neighbor would be out of bounds (lower)
                continue;
            };
            if touched_positions.contains(&(neighbor_position, *dir, node.straightness)) {
                // we've been here before
                continue;
            }
            if same_direction && node.straightness >= 3 {
                // can't go straight for mode than 3 tiles
                continue;
            }
            let Some(position_cost) = map.at(neighbor_position) else {
                // neighbor would be out of bounds (upper)
                continue;
            };
            touched_positions.insert((neighbor_position, *dir, new_straightness));
            let new_node = SearchNode::new(
                map,
                Some(Box::new(node.clone())),
                neighbor_position,
                node.g + position_cost as usize,
                *dir,
                new_straightness,
            );
            heap.push(new_node);
        }
    }
}

fn solve_part2(map: &Map, start: Position) -> anyhow::Result<usize> {
    let goal = Position::new(map.width - 1, map.height - 1);
    let mut touched_positions: HashSet<(Position, Direction, usize)> = HashSet::new();

    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();
    heap.push(SearchNode::new(map, None, start, 0, Direction::Right, 0));
    touched_positions.insert((start, Direction::Right, 0));

    loop {
        let Some(node) = heap.pop() else {
            return Err(anyhow!("A* ran out of search nodes"));
        };
        // println!("Checking {:?}", node.pos);

        if node.pos == goal {
            print_final_path(map, &node);
            return Ok(node.g);
        }

        for dir in DIRECTION_PRIORITY {
            if node.facing.opposite() == *dir {
                // can't turn 180 degrees
                continue;
            }
            let same_direction = *dir == node.facing;
            let new_straightness = if same_direction {
                node.straightness + 1
            } else {
                1
            };
            let Some(neighbor_position) = node.pos.towards(*dir) else {
                // neighbor would be out of bounds (lower)
                continue;
            };
            if touched_positions.contains(&(neighbor_position, *dir, node.straightness)) {
                // we've been here before
                continue;
            }
            if !same_direction && node.straightness < 4 {
                continue;
            }
            // must go straight for at least 4 tiles
            if same_direction && node.straightness >= 10 {
                // can't go straight for mode than 10 tiles
                continue;
            }
            let Some(position_cost) = map.at(neighbor_position) else {
                // neighbor would be out of bounds (upper)
                continue;
            };
            touched_positions.insert((neighbor_position, *dir, new_straightness));
            let new_node = SearchNode::new(
                map,
                Some(Box::new(node.clone())),
                neighbor_position,
                node.g + position_cost as usize,
                *dir,
                new_straightness,
            );
            heap.push(new_node);
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = Map::parse(lines)?;
    solve(&map, Position::new(0, 0))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = Map::parse(lines)?;
    solve_part2(&map, Position::new(0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];

    static EXAMPLE_2: &[&str] = &[
        "111111111111",
        "999999999991",
        "999999999991",
        "999999999991",
        "999999999991",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 102);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();
        assert_eq!(result, 94);

        let result2 = part2(EXAMPLE_2).unwrap();
        assert_eq!(result2, 71);
    }
}
