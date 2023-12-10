use std::fmt::Display;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
    Any,
}

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
    Ground,
    Start,
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
            '.' => Ok(Tile::Ground),
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
            Self::Ground => None,
            Self::Start => Some(Direction::Any),
        }
    }
}

#[derive(Debug, Default)]
struct Maze {
    inner: Vec<Vec<Tile>>,
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn traverse_node(
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
    maze: &Maze,
    start: Position,
    facing: Direction,
) -> anyhow::Result<usize> {
    let (mut current, mut facing) = traverse_node(maze, start, facing)?;
    let mut steps = 1;

    while current != start {
        (current, facing) = traverse_node(maze, current, facing)?;
        steps += 1;
    }

    Ok(steps)
}

fn traverse_maze(maze: &Maze, start: Position) -> anyhow::Result<usize> {
    for starting_direction in &[
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ] {
        let path_length = traverse_maze_starting_towards(maze, start, *starting_direction);
        if path_length.is_ok() {
            return path_length;
        } else {
            continue;
        }
    }

    Err(anyhow!(
        "Could not traverse maze from starting point in either direction"
    ))
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<usize>
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

    let loop_length = traverse_maze(&maze, start)?;

    Ok(loop_length / 2)
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

    static EXAMPLE: &[&str] = &["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
