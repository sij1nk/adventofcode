use std::convert::TryInto;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum ParseUniverseError {
    Value,
}

impl fmt::Display for ParseUniverseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Value => write!(f, "ParseUniverseError::Value"),
        }
    }
}

impl Error for ParseUniverseError {}

struct Universe {
    planes: Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    z: usize,
}

impl Universe {
    fn new<'a, I, S>(lines: I, cycles: usize) -> Result<Self, ParseUniverseError>
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut cells = vec![];

        let (mut x, mut y) = (0, 0);
        for line in lines.into_iter().map(|l| l.as_ref()) {
            for c in line.chars() {
                match c {
                    '.' => cells.push(Cell::Inactive),
                    '#' => cells.push(Cell::Active),
                    _ => return Err(ParseUniverseError::Value),
                }
                x += 1;
            }
            y += 1;
        }

        // Assuming lines are of equal width
        x = x / y;
        let new_x = x + 2 * cycles;
        let new_y = y + 2 * cycles;
        let z = 1 + 2 * cycles;

        let empty_plane = vec![Cell::Inactive; new_x * new_y];
        let mut planes = vec![empty_plane.clone(); z];

        let mut starting_plane = empty_plane.clone();
        cells
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| match c {
                Cell::Active => Some((i, c)),
                _ => None,
            })
            .for_each(|(i, c)| {
                let idx = new_x * ((new_y - y) / 2 + i / x) + (new_x - x) / 2 + i % x;
                starting_plane[idx] = c;
            });

        planes[cycles] = starting_plane;

        Ok(Universe {
            planes,
            x: new_x,
            y: new_y,
            z,
        })
    }

    pub fn get_cell_count(&self, cell_type: Cell) -> usize {
        self.planes
            .iter()
            .map(|p| p.iter().filter(|&c| *c == cell_type).count())
            .sum()
    }

    pub fn tick(&mut self) {
        let mut new_planes = self.planes.clone();

        for z in 0..self.planes.len() {
            for i in 0..self.x * self.y {
                let neighbors = self.get_neighbors(z, i);
                let cell = self.get_cell(z, i);

                match *cell {
                    Cell::Inactive => {
                        if neighbors.iter().filter(|&&n| *n == Cell::Active).count() == 3 {
                            new_planes[z][i] = Cell::Active;
                        }
                    }
                    Cell::Active => {
                        let count = neighbors.iter().filter(|&&n| *n == Cell::Active).count();
                        if count != 2 && count != 3 {
                            new_planes[z][i] = Cell::Inactive;
                        }
                    }
                }
            }
        }

        self.planes = new_planes;
    }

    fn get_cell(&self, z: usize, i: usize) -> &Cell {
        &self.planes[z][i]
    }

    fn get_neighbors(&self, z: usize, i: usize) -> Vec<&Cell> {
        let z: i32 = z.try_into().unwrap(); // Safe because z is small
        let mut neighbors = vec![];
        for z_offset in [-1, 0, 1].into_iter() {
            let z_idx = z + z_offset;
            if z_idx < 0 || self.planes.len() <= z_idx as usize {
                continue;
            }

            if let Some(plane) = self.planes.get(z_idx as usize) {
                let left = i % self.x != 0;
                let right = i % self.x != self.x - 1;
                let up = i >= self.x;
                let down = i <= (self.y - 1) * self.x - 1;

                if left {
                    neighbors.push(&plane[i - 1]);
                }

                if right {
                    neighbors.push(&plane[i + 1]);
                }

                if up {
                    neighbors.push(&plane[i - self.x]);
                }

                if down {
                    neighbors.push(&plane[i + self.x]);
                }

                if left && up {
                    neighbors.push(&plane[i - self.x - 1]);
                }

                if right && up {
                    neighbors.push(&plane[i - self.x + 1]);
                }

                if left && down {
                    neighbors.push(&plane[i + self.x - 1]);
                }

                if right && down {
                    neighbors.push(&plane[i + self.x + 1]);
                }

                if *z_offset != 0 {
                    neighbors.push(&plane[i]);
                }
            }
        }

        neighbors
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut z: i32 = self.z.try_into().unwrap();
        z /= 2;
        write!(f, "{}x{}, height: {}\n", self.x, self.y, self.z)?;
        for plane in self.planes.iter() {
            write!(f, "Plane {}:\n", z)?;
            for row in plane.chunks(self.x) {
                for cell in row {
                    write!(f, "{}", cell)?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;

            z -= 1;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Active,
    Inactive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Active => write!(f, "# "),
            Cell::Inactive => write!(f, ". "),
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut u = Universe::new(lines, 6).ok()?;

    for _ in 0..6 {
        u.tick();
    }

    Some(u.get_cell_count(Cell::Active))
}

pub fn part2<'a, I, S>(_lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[".#.", "..#", "###"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 112);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
