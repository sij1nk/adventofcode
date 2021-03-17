use std::convert::TryInto;
use std::fmt;
use std::ops::{Add, Sub};

enum Part {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Universe {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    part: Part,
}

impl Universe {
    pub fn new<'a, I, S>(lines: I, part: Part) -> Universe
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut cells = Vec::new();

        let mut height = 0;
        for line in lines.into_iter() {
            let line = line.as_ref();
            for c in line.chars() {
                match c {
                    '.' => cells.push(Cell::Void),
                    'L' => cells.push(Cell::Empty),
                    '#' => cells.push(Cell::Taken),
                    _ => (),
                }
            }
            height += 1;
        }

        // Assuming lines are of equal width
        let width = cells.len() / height;

        Universe {
            cells,
            width,
            height,
            part,
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();
        let tolerance = match &self.part {
            Part::One => 4,
            Part::Two => 5,
        };

        for i in 0..self.cells.len() {
            let neighbors = match self.part {
                Part::One => self.get_direct_neighbors(i),
                Part::Two => self.get_visible_neighbors(i),
            };

            if self.cells[i] != Cell::Void {
                if neighbors.iter().all(|&c| *c != Cell::Taken) {
                    new_cells[i] = Cell::Taken;
                } else if neighbors.iter().filter(|&&c| *c == Cell::Taken).count() >= tolerance {
                    new_cells[i] = Cell::Empty;
                }
            }
        }

        self.cells = new_cells;
    }

    pub fn get_cell_count(&self, cell_type: Cell) -> usize {
        self.cells.iter().filter(|&c| *c == cell_type).count()
    }

    fn idx_to_vector(&self, idx: usize) -> Vector {
        Vector {
            x: (idx % self.width) as i32,
            y: (idx / self.width) as i32,
        }
    }

    fn vector_to_idx(&self, v: Vector) -> Option<usize> {
        let iwidth: i32 = self.width.try_into().ok()?;
        (v.y * iwidth + v.x).try_into().ok()
    }

    fn get_cell(&self, v: Vector) -> Option<&Cell> {
        if v.x >= 0
            && v.x < self.width.try_into().ok()?
            && v.y >= 0
            && v.y < self.height.try_into().ok()?
        {
            let idx = self.vector_to_idx(v)?;
            return Some(&self.cells[idx]);
        }

        None
    }

    fn get_visible_neighbors(&self, idx: usize) -> Vec<&Cell> {
        let deltas = [
            Vector { x: -1, y: -1 },
            Vector { x: -1, y: 0 },
            Vector { x: -1, y: 1 },
            Vector { x: 0, y: -1 },
            Vector { x: 0, y: 1 },
            Vector { x: 1, y: -1 },
            Vector { x: 1, y: 0 },
            Vector { x: 1, y: 1 },
        ];
        let original_cell_vector = self.idx_to_vector(idx);
        let mut neighbors = vec![];

        for delta in deltas.iter() {
            let mut new_vector = original_cell_vector;
            loop {
                new_vector = new_vector + *delta;
                match self.get_cell(new_vector) {
                    Some(cell) => match *cell {
                        Cell::Void => continue,
                        _ => {
                            neighbors.push(cell);
                            break;
                        }
                    },
                    None => break,
                };
            }
        }

        neighbors
    }

    fn get_direct_neighbors(&self, idx: usize) -> Vec<&Cell> {
        // This panics if cells.len() > i32::MAX, which is not the case with sensible puzzle inputs
        let width: i32 = self.width.try_into().unwrap();
        let mut relative_indices = vec![-width, width];
        if idx % self.width != 0 {
            relative_indices.push(-width - 1);
            relative_indices.push(-1);
            relative_indices.push(width - 1);
        }
        if idx % self.width != self.width - 1 {
            relative_indices.push(-width + 1);
            relative_indices.push(1);
            relative_indices.push(width + 1);
        }

        relative_indices
            .iter()
            .filter_map(|&i| {
                // This looks dumb
                let ui: usize = (i.checked_add(idx.try_into().ok()?))?.try_into().ok()?;
                self.cells.get(ui)
            })
            .collect::<Vec<_>>()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.chunks(self.width) {
            for cell in line.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Void,
    Empty,
    Taken,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Void => '.',
                Cell::Empty => 'L',
                Cell::Taken => '#',
            }
        )
    }
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut u = Universe::new(lines, Part::One);
    let mut takens = 0;

    loop {
        u.tick();
        let new_takens = u.get_cell_count(Cell::Taken);
        if takens == new_takens {
            break;
        } else {
            takens = new_takens;
        }
    }

    takens
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut u = Universe::new(lines, Part::Two);
    let mut takens = 0;

    loop {
        u.tick();
        let new_takens = u.get_cell_count(Cell::Taken);
        if takens == new_takens {
            break;
        } else {
            takens = new_takens;
        }
    }

    takens
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);

        assert_eq!(result, 37);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);

        assert_eq!(result, 26);
    }
}
