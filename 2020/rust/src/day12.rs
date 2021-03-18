use crate::util;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseInstructionError {
    Action,
    Value,
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Action => write!(f, "ParseInstructionError::Action"),
            Self::Value => write!(f, "ParseInstructionError::Value"),
        }
    }
}

impl error::Error for ParseInstructionError {}

#[derive(Debug)]
enum InstructionType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    action: InstructionType,
    value: f64,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.action, self.value)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action_str, value_str) = s.split_at(1);
        let action = match action_str.parse::<char>().map_err(|_| Self::Err::Action)? {
            'N' => InstructionType::North,
            'S' => InstructionType::South,
            'E' => InstructionType::East,
            'W' => InstructionType::West,
            'L' => InstructionType::Left,
            'R' => InstructionType::Right,
            'F' => InstructionType::Forward,
            _ => return Err(Self::Err::Action),
        };
        let value = value_str.parse::<f64>().map_err(|_| Self::Err::Value)?;
        Ok(Instruction { action, value })
    }
}

fn rotate_point(point: (f64, f64), angle: f64) -> (f64, f64) {
    let angle = angle.to_radians();
    let (x, y) = point;
    let s: f64 = angle.sin();
    let c: f64 = angle.cos();

    // always rotate around (0, 0) origin
    let new_x = x * c - y * s;
    let new_y = x * s + y * c;
    (new_x, new_y)
}

pub fn part1<'a, I, S>(lines: I) -> Result<i64, ParseInstructionError>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (mut x, mut y) = (0.0, 0.0);
    let mut dir = 0.0;
    let instructions: Vec<Instruction> = util::parse_many(lines)?;

    for ins in instructions.iter() {
        match ins.action {
            InstructionType::North => y += ins.value,
            InstructionType::South => y -= ins.value,
            InstructionType::East => x += ins.value,
            InstructionType::West => x -= ins.value,
            InstructionType::Left => dir += ins.value,
            InstructionType::Right => dir -= ins.value,
            InstructionType::Forward => {
                x += ins.value * dir.to_radians().cos();
                y += ins.value * dir.to_radians().sin();
            }
        }
    }

    Ok((x.round().abs() + y.round().abs()) as i64)
}

pub fn part2<'a, I, S>(lines: I) -> Result<i64, ParseInstructionError>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (mut x, mut y) = (0.0, 0.0);
    let (mut wp_x, mut wp_y) = (10.0, 1.0);
    let instructions: Vec<Instruction> = util::parse_many(lines)?;

    for ins in instructions.iter() {
        match ins.action {
            InstructionType::North => wp_y += ins.value,
            InstructionType::South => wp_y -= ins.value,
            InstructionType::East => wp_x += ins.value,
            InstructionType::West => wp_x -= ins.value,
            InstructionType::Left => {
                let point = rotate_point((wp_x, wp_y), ins.value);
                wp_x = point.0;
                wp_y = point.1;
            }
            InstructionType::Right => {
                let point = rotate_point((wp_x, wp_y), -ins.value);
                wp_x = point.0;
                wp_y = point.1;
            }
            InstructionType::Forward => {
                x += ins.value * wp_x;
                y += ins.value * wp_y;
            }
        }
    }
    Ok((x.round().abs() + y.round().abs()) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["F10", "N3", "F7", "R90", "F11"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 25);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 286);
    }
}
