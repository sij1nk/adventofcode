use std::{collections::BTreeMap, fmt::Display};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
    Any,
}

const DIRECTION_PRIORITY: &[Direction] = &[
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::North => write!(f, "North"),
            Self::West => write!(f, "West"),
            Self::South => write!(f, "South"),
            Self::East => write!(f, "East"),
            Self::Any => write!(f, "Any"),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::Any => Self::Any,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Ground { fake: bool },
    Start,
}

#[derive(Debug, Clone, Copy)]
struct GroundProperties {
    fake: bool,
    edge: bool,
}

impl Tile {
    fn parse(c: char) -> anyhow::Result<Self> {
        match c {
            '|' => Ok(Tile::Pipe(Direction::North, Direction::South)),
            '-' => Ok(Tile::Pipe(Direction::East, Direction::West)),
            'L' => Ok(Tile::Pipe(Direction::North, Direction::East)),
            'J' => Ok(Tile::Pipe(Direction::North, Direction::West)),
            '7' => Ok(Tile::Pipe(Direction::South, Direction::West)),
            'F' => Ok(Tile::Pipe(Direction::South, Direction::East)),
            '.' => Ok(Tile::Ground { fake: false }),
            'S' => Ok(Tile::Start),
            unknown => Err(anyhow!("Found unknown tile type '{}'", unknown)),
        }
    }

    fn get_next_direction(&self, from_dir: Direction) -> Option<Direction> {
        match *self {
            Self::Pipe(d1, d2) => {
                if from_dir == d1 {
                    Some(d2)
                } else if from_dir == d2 {
                    Some(d1)
                } else {
                    None
                }
            }
            Self::Ground { fake: _ } => None,
            Self::Start => Some(Direction::Any),
        }
    }
}

#[derive(Debug, Default)]
struct Maze {
    inner: Vec<Vec<Tile>>,
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for tile in row {
                match *tile {
                    Tile::Start => write!(f, "S"),
                    Tile::Pipe(d1, d2) => {
                        if d1 == Direction::North && d2 == Direction::South {
                            write!(f, "|")
                        } else if d1 == Direction::East && d2 == Direction::West {
                            write!(f, "-")
                        } else if d1 == Direction::North && d2 == Direction::East {
                            write!(f, "L")
                        } else if d1 == Direction::North && d2 == Direction::West {
                            write!(f, "J")
                        } else if d1 == Direction::South && d2 == Direction::West {
                            write!(f, "7")
                        } else {
                            write!(f, "F")
                        }
                    }
                    Tile::Ground { fake: false } => write!(f, "."),
                    Tile::Ground { fake: true } => write!(f, " "),
                }?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Maze {
    fn push(&mut self, row: Vec<Tile>) {
        self.inner.push(row);
    }

    fn get_tile(&self, p: Position) -> Option<Tile> {
        let row = self.inner.get(p.y)?;
        let tile = row.get(p.x);
        tile.copied()
    }

    fn get_tile_mut(&mut self, p: Position) -> Option<&mut Tile> {
        self.inner.get_mut(p.y)?.get_mut(p.x)
    }

    fn is_edge_position(&self, p: Position) -> bool {
        p.x == 0 || p.y == 0 || p.y == self.inner.len() - 1 || p.x == self.inner[0].len() - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_next_position(p: Position, facing: Direction) -> Option<Position> {
    let next_position = match facing {
        Direction::North => Position {
            x: p.x,
            y: p.y.checked_sub(1)?,
        },
        Direction::West => Position {
            x: p.x.checked_sub(1)?,
            y: p.y,
        },
        Direction::South => Position { x: p.x, y: p.y + 1 },
        Direction::East => Position { x: p.x + 1, y: p.y },
        Direction::Any => return None,
    };

    Some(next_position)
}

fn traverse_tile(
    maze: &Maze,
    current: Position,
    facing: Direction,
) -> anyhow::Result<(Position, Direction)> {
    let next_position = get_next_position(current, facing).ok_or(anyhow!(
        "Position to the {} of {} is out of bounds",
        facing,
        current
    ))?;
    let next_tile = maze
        .get_tile(next_position)
        .ok_or(anyhow!("Couldn't get tile at position {}", next_position))?;
    if let Some(next_dir) = next_tile.get_next_direction(facing.opposite()) {
        Ok((next_position, next_dir))
    } else {
        Err(anyhow!(
            "Next tile doesn't support traversal from this direction"
        ))
    }
}

fn traverse_maze_starting_towards(
    maze: &mut Maze,
    start: Position,
    start_direction: Direction,
) -> anyhow::Result<Vec<Position>> {
    let mut loop_positions = Vec::new();
    loop_positions.push(start);

    let mut old_facing = Direction::Any;
    let (mut current, mut facing) = traverse_tile(maze, start, start_direction)?;
    loop_positions.push(current);

    while current != start {
        old_facing = facing;
        (current, facing) = traverse_tile(maze, current, facing)?;
        loop_positions.push(current);
    }

    let Some(start_tile) = maze.get_tile_mut(start) else {
        return Err(anyhow!("Could not find start tile"));
    };

    *start_tile = Tile::Pipe(old_facing.opposite(), start_direction);

    Ok(loop_positions)
}

fn traverse_maze(maze: &mut Maze, start: Position) -> anyhow::Result<Vec<Position>> {
    for starting_direction in &[
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ] {
        let loop_positions = traverse_maze_starting_towards(maze, start, *starting_direction);
        if loop_positions.is_ok() {
            return loop_positions;
        } else {
            continue;
        }
    }

    Err(anyhow!(
        "Could not traverse maze from starting point in either direction"
    ))
}

fn parse_maze<'a, I, S>(lines: I) -> anyhow::Result<(Maze, Position)>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut maze = Maze::default();
    let mut start: Option<Position> = None;

    for (y, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        let mut row = vec![];
        for (x, char) in line.chars().enumerate() {
            let tile = Tile::parse(char)?;

            if tile == Tile::Start {
                if start.is_some() {
                    return Err(anyhow!("Found multiple starting positions in input"));
                }
                start = Some(Position { x, y });
            }

            row.push(tile);
        }
        maze.push(row);
    }

    let Some(start) = start else {
        return Err(anyhow!("Found no starting positions in input"));
    };

    Ok((maze, start))
}

fn extrapolate_maze(maze: &mut Maze, loop_positions: &mut [Position]) {
    let mut new_rows = Vec::with_capacity(maze.inner.len() * 2 - 1);

    for (y, row) in maze.inner.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len() * 2 - 1);
        let mut extrapolated_row = Vec::with_capacity(row.len() * 2 - 1);

        for (x, &tile) in row.iter().enumerate() {
            let pos = Position { x, y };

            if loop_positions.contains(&pos) {
                let tile_after = match tile {
                    Tile::Pipe(d1, d2) if d1 == Direction::East || d2 == Direction::East => {
                        Tile::Pipe(Direction::East, Direction::West)
                    }
                    _ => Tile::Ground { fake: true },
                };

                let tile_below = match tile {
                    Tile::Pipe(d1, d2) if d1 == Direction::South || d2 == Direction::South => {
                        Tile::Pipe(Direction::North, Direction::South)
                    }
                    _ => Tile::Ground { fake: true },
                };

                new_row.push(tile);
                new_row.push(tile_after);
                extrapolated_row.push(tile_below);
            } else {
                new_row.push(Tile::Ground { fake: false });
                new_row.push(Tile::Ground { fake: true });
                extrapolated_row.push(Tile::Ground { fake: true });
            }

            extrapolated_row.push(Tile::Ground { fake: true });
        }

        new_rows.push(new_row);
        new_rows.push(extrapolated_row);
    }

    let new_maze = Maze { inner: new_rows };
    *maze = new_maze;
}

fn collect_ground_positions(maze: &Maze) -> BTreeMap<Position, GroundProperties> {
    let mut positions = BTreeMap::new();

    for (y, row) in maze.inner.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Ground { fake } = tile {
                let p = Position { x, y };
                positions.insert(
                    p,
                    GroundProperties {
                        fake: *fake,
                        edge: maze.is_edge_position(p),
                    },
                );
            }
        }
    }

    positions
}

// Perform depth-first search on connected ground positions. Return the number checked non-fake
// ground positions, and whether the group of checked positions touched the maze edge
fn dfs(
    pos: Position,
    positions_to_check: &mut BTreeMap<Position, GroundProperties>,
) -> (usize, bool) {
    let Some(GroundProperties { fake, edge }) = positions_to_check.remove(&pos) else {
        panic!("Start position was not in positions_to_check");
    };
    let mut checked = if fake { 0 } else { 1 };
    let mut touched_edge = edge;

    for &d in DIRECTION_PRIORITY {
        let Some(next) = get_next_position(pos, d) else {
            // Position is on the edge of the maze
            touched_edge = true;
            continue;
        };

        if positions_to_check.get(&next).is_none() {
            // Position does not need to be checked - it's not ground, or it has been checked before
            continue;
        }

        let (child_checked, child_touched_edge) = dfs(next, positions_to_check);
        touched_edge |= child_touched_edge;
        if touched_edge {
            checked = 0;
        } else {
            checked += child_checked;
        }
    }

    (checked, touched_edge)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (mut maze, start) = parse_maze(lines)?;
    let loop_positions = traverse_maze(&mut maze, start)?;

    Ok(loop_positions.len() / 2)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (mut maze, start) = parse_maze(lines)?;
    let mut loop_positions = traverse_maze(&mut maze, start)?;
    extrapolate_maze(&mut maze, &mut loop_positions);

    let mut ground_positions = collect_ground_positions(&maze);

    let mut real_ground_tiles_contained_by_loop = 0;

    while !ground_positions.is_empty() {
        let Some(&ground) = ground_positions.keys().next() else {
            break;
        };
        let (checked_real_tiles, tiles_touched_maze_edge) = dfs(ground, &mut ground_positions);
        if !tiles_touched_maze_edge {
            real_ground_tiles_contained_by_loop += checked_real_tiles;
        }
    }

    Ok(real_ground_tiles_contained_by_loop)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];

    static EXAMPLE_PART2: &[&str] = &[
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ];

    static EXAMPLE_PART2_2: &[&str] = &[
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];

    static EXAMPLE_PART2_3: &[&str] = &[
        "..........",
        ".S------7.",
        ".|F----7|.",
        ".||....||.",
        ".||....||.",
        ".|L-7F-J|.",
        ".|..||..|.",
        ".L--JL--J.",
        "..........",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let result1 = part2(EXAMPLE_PART2).unwrap();

        assert_eq!(result1, 8);

        let result2 = part2(EXAMPLE_PART2_2).unwrap();

        assert_eq!(result2, 10);

        let result3 = part2(EXAMPLE_PART2_3).unwrap();

        assert_eq!(result3, 4);
    }
}
