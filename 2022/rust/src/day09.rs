use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum VDir {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
enum HDir {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Direction {
    v: Option<VDir>,
    h: Option<HDir>,
}

impl Direction {
    pub fn new(v: Option<VDir>, h: Option<HDir>) -> Self {
        Self { v, h }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Self::new(Some(VDir::Up), None)),
            'D' => Some(Self::new(Some(VDir::Down), None)),
            'L' => Some(Self::new(None, Some(HDir::Left))),
            'R' => Some(Self::new(None, Some(HDir::Right))),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn mov(&mut self, dir: Direction) {
        if let Some(vdir) = dir.v {
            match vdir {
                VDir::Up => self.y -= 1,
                VDir::Down => self.y += 1,
            }
        }
        if let Some(hdir) = dir.h {
            match hdir {
                HDir::Left => self.x -= 1,
                HDir::Right => self.x += 1,
            }
        }
    }

    // which way do we have to go towards other
    pub fn d(&self, other: &Point) -> (i32, Direction) {
        let mut vdir = None;
        let mut hdir = None;

        let dx = self.x - other.x;
        let dy = self.y - other.y;

        let mut d = dx.abs() + dy.abs();

        if dx != 0 && dy != 0 {
            d -= 1;
        }

        if dx != 0 {
            hdir = if dx < 0 {
                Some(HDir::Right)
            } else {
                Some(HDir::Left)
            };
        };

        if dy != 0 {
            vdir = if dy < 0 {
                Some(VDir::Down)
            } else {
                Some(VDir::Up)
            };
        };

        let dir = Direction::new(vdir, hdir);

        (d, dir)
    }
}

struct Rope {
    knots: Vec<Point>,
    tail_touched: HashSet<Point>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut tail_touched = HashSet::new();
        let start = Point::new(0, 0);

        let knots = vec![start; length];
        tail_touched.insert(start);

        Self {
            knots,
            tail_touched,
        }
    }

    pub fn mov(&mut self, dir: Direction, length: i32) {
        for _ in 0..length {
            let head = &mut self.knots[0];
            head.mov(dir);

            let len = self.knots.len();

            for i in 1..len {
                let cur = self.knots[i];
                let (d, dir) = cur.d(&self.knots[i - 1]);

                let cur = &mut self.knots[i];
                if d > 1 {
                    cur.mov(dir);
                }

                if i == len - 1 {
                    self.tail_touched.insert(*cur);
                }
            }
        }
    }

    pub fn number_of_tiles_touched(&self) -> usize {
        self.tail_touched.len()
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut state = Rope::new(2);

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (dir, length) = line.split_once(' ').unwrap();
        let dir = Direction::from_char(dir.chars().next().unwrap()).unwrap();
        let length = length.parse::<i32>().unwrap();

        state.mov(dir, length);
    }

    Some(state.number_of_tiles_touched())
}

pub fn part2<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut state = Rope::new(10);

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (dir, length) = line.split_once(' ').unwrap();
        let dir = Direction::from_char(dir.chars().next().unwrap()).unwrap();
        let length = length.parse::<i32>().unwrap();

        state.mov(dir, length);
    }

    Some(state.number_of_tiles_touched())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();
        println!("{}", result);

        assert_eq!(result, 1);
    }
}
