use itertools::Itertools;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
enum ParseMaskError {
    Length,
    Value,
}

impl fmt::Display for ParseMaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseMaskError::Length => write!(f, "ParseMaskError::Length"),
            ParseMaskError::Value => write!(f, "ParseMaskError::Value"),
        }
    }
}

impl Error for ParseMaskError {}

#[derive(Debug)]
struct ValueMask {
    zeros: u64,
    ones: u64,
}

impl FromStr for ValueMask {
    type Err = ParseMaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = ValueMask {
            zeros: u64::MAX,
            ones: 0,
        };
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => mask.zeros &= !(0b1 << i),
                '1' => mask.ones |= 0b1 << i,
                'X' => continue,
                _ => return Err(ParseMaskError::Value),
            };

            if i >= 36 {
                return Err(ParseMaskError::Length);
            }
        }

        Ok(mask)
    }
}

struct AddressMask {
    zeros: u64,
    ones: u64,
    floating: Vec<u64>,
}

impl FromStr for AddressMask {
    type Err = ParseMaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = AddressMask {
            zeros: u64::MAX,
            ones: 0,
            floating: vec![],
        };
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => continue,
                '1' => mask.ones |= 0b1 << i,
                'X' => {
                    mask.zeros &= !(0b1 << i);
                    mask.floating.push(2u64.pow(i.try_into().unwrap()));
                }
                _ => return Err(ParseMaskError::Value),
            };

            if i >= 36 {
                return Err(ParseMaskError::Length);
            }
        }

        Ok(mask)
    }
}

struct Computer {
    value_mask: Option<ValueMask>,
    address_mask: Option<AddressMask>,
    memory: BTreeMap<u64, u64>,
    mode: Part,
}

impl Computer {
    pub fn new(mode: Part) -> Computer {
        Computer {
            value_mask: None,
            address_mask: None,
            memory: BTreeMap::new(),
            mode,
        }
    }

    pub fn set_value_mask(&mut self, mask: ValueMask) {
        self.value_mask = Some(mask);
    }

    pub fn set_address_mask(&mut self, mask: AddressMask) {
        self.address_mask = Some(mask);
    }

    pub fn insert(&mut self, address: u64, value: u64) {
        match self.mode {
            Part::One => self.insert_with_value_mask(address, value),
            Part::Two => self.insert_with_address_mask(address, value),
        }
    }

    fn insert_with_value_mask(&mut self, address: u64, mut value: u64) {
        if let Some(mask) = &self.value_mask {
            value = value & mask.zeros;
            value = value | mask.ones;
        }
        self.memory.insert(address, value);
    }

    fn insert_with_address_mask(&mut self, mut address: u64, value: u64) {
        if let Some(mask) = &self.address_mask {
            address = address & mask.zeros;
            address = address | mask.ones;

            for f in mask.floating.iter().powerset() {
                let sum = f.into_iter().sum::<u64>();
                self.memory.insert(address + sum, value);
            }
        }
    }

    pub fn get_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut computer = Computer::new(Part::One);
    for line in lines.into_iter() {
        let mut splits = line.as_ref().split(' ');
        let word = splits.next()?;

        if let Some(word) = word.strip_prefix("mem[") {
            let word = word.strip_suffix("]")?;
            let address = word.parse::<u64>().ok()?;
            let value = splits.last()?.parse::<u64>().ok()?;
            computer.insert(address, value);
        } else if let Some(_) = word.strip_prefix("mask") {
            let mask = ValueMask::from_str(splits.last()?).ok()?;
            computer.set_value_mask(mask);
        }
    }

    Some(computer.get_sum())
}

pub fn part2<'a, I, S>(lines: I) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut computer = Computer::new(Part::Two);
    for line in lines.into_iter() {
        let mut splits = line.as_ref().split(' ');
        let word = splits.next()?;

        if let Some(word) = word.strip_prefix("mem[") {
            let word = word.strip_suffix("]")?;
            let address = word.parse::<u64>().ok()?;
            let value = splits.last()?.parse::<u64>().ok()?;
            computer.insert(address, value);
        } else if let Some(_) = word.strip_prefix("mask") {
            let mask = AddressMask::from_str(splits.last()?).ok()?;
            computer.set_address_mask(mask);
        }
    }

    Some(computer.get_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART1_EXAMPLE: &[&str] = &[
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ];

    static PART2_EXAMPLE: &[&str] = &[
        "mask = 000000000000000000000000000000X1001X",
        "mem[42] = 100",
        "mask = 00000000000000000000000000000000X0XX",
        "mem[26] = 1",
    ];

    #[test]
    fn part1_test() {
        let result = part1(PART1_EXAMPLE).unwrap();

        assert_eq!(result, 165);
    }

    #[test]
    fn part2_test() {
        let result = part2(PART2_EXAMPLE).unwrap();

        assert_eq!(result, 208);
    }
}
