use std::collections::{BinaryHeap, HashSet};

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
    data: Vec<u32>,
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

        let to_chars = |l: &'a str| l.chars().filter_map(|c| c.to_digit(10).map(|d| d as u32));

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

    fn at(&self, pos: Position) -> Option<u32> {
        if pos.x >= self.width || pos.y >= self.height {
            None
        } else {
            let i = pos.y * self.width + pos.x;
            self.data.get(i).copied()
        }
    }
}

fn heuristic(map: &Map, pos: Position) -> usize {
    pos.distance(Position::new(map.width - 1, map.height - 1))
}

#[derive(Debug, Clone)]
struct SearchNode {
    pos: Position,
    g: usize,
    h: usize,
    facing: Direction,
    straightness: usize,
}

impl SearchNode {
    fn new(map: &Map, pos: Position, g: usize, facing: Direction, straightness: usize) -> Self {
        Self {
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

fn solve_astar(
    map: &Map,
    start: Position,
    goal: Position,
    exit_condition: impl Fn(&SearchNode, &Position) -> bool,
    custom_constraint: impl Fn(&SearchNode, &Direction) -> bool,
) -> anyhow::Result<usize> {
    let mut touched_positions: HashSet<(Position, Direction, usize)> = HashSet::new();

    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();
    heap.push(SearchNode::new(map, start, 0, Direction::Right, 0));
    touched_positions.insert((start, Direction::Right, 0));

    loop {
        let Some(node) = heap.pop() else {
            return Err(anyhow!("Pathfinding ran out of search nodes"));
        };

        if exit_condition(&node, &goal) {
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
            if touched_positions.contains(&(neighbor_position, *dir, new_straightness)) {
                // we've been here before
                continue;
            }
            if custom_constraint(&node, dir) {
                continue;
            }
            let Some(position_cost) = map.at(neighbor_position) else {
                // neighbor would be out of bounds (upper)
                continue;
            };
            touched_positions.insert((neighbor_position, *dir, new_straightness));
            let new_node = SearchNode::new(
                map,
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
    let start = Position::new(0, 0);
    let goal = Position::new(map.width - 1, map.height - 1);
    let exit_condition = |node: &SearchNode, goal: &Position| node.pos == *goal;
    let custom_constraint =
        |node: &SearchNode, facing: &Direction| node.facing == *facing && node.straightness >= 3;
    solve_astar(&map, start, goal, exit_condition, custom_constraint)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let map = Map::parse(lines)?;
    let start = Position::new(0, 0);
    let goal = Position::new(map.width - 1, map.height - 1);
    let exit_condition =
        |node: &SearchNode, goal: &Position| node.pos == *goal && node.straightness >= 4;
    let custom_constraint = |node: &SearchNode, facing: &Direction| {
        node.facing == *facing && node.straightness >= 10
            || node.facing != *facing && node.straightness < 4
    };
    solve_astar(&map, start, goal, exit_condition, custom_constraint)
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

    static EXAMPLE_3: &[&str] = &[
        "199999999999",
        "199999999999",
        "199999999999",
        "199999999999",
        "111111111111",
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
    }

    #[test]
    fn part2_test_unfortunate_path() {
        let result = part2(EXAMPLE_2).unwrap();
        assert_eq!(result, 71);
    }

    #[test]
    fn part2_test_unfortunate_path2() {
        let result = part2(EXAMPLE_3).unwrap();
        assert_eq!(result, 71);
    }
}
