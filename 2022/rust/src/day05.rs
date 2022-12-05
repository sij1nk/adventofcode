use std::{str::Chars, iter::{Skip, StepBy}};

use regex::{Regex, Captures};

use crate::util;

struct MoveCommand {
    amount: usize,
    from: usize,
    to: usize
}

enum Part {
    One,
    Two
}

#[derive(Debug)]
pub struct Data {
    stacks: Vec<Vec<char>>
}

impl Data {
    fn parse(lines: &mut [StepBy<Skip<Chars>>]) -> Self {
        let mut data = Data { stacks: vec![] };

        'outer: loop {
            let mut s = String::from("");

            for line in lines.iter_mut() {
                if let Some(c) = line.next() {
                    if c.is_alphabetic() {
                        s.push(c);
                    }
                } else {
                    break 'outer;
                }
            }

            data.create_stack(&s);

            s.clear();
        }

        data
    }

    fn create_stack(&mut self, items: &str) {
        self.stacks.push(items.chars().rev().collect::<Vec<_>>());
    }

    fn mov(&mut self, cmd: MoveCommand, part: Part) {
        let from = self.stacks.get_mut(cmd.from).unwrap();
        let drain = from.drain((from.len() - cmd.amount)..).collect::<Vec<_>>();

        let to = self.stacks.get_mut(cmd.to).unwrap();
        match part {
            Part::One => to.extend(drain.iter().rev()),
            Part::Two => to.extend(drain.iter())
        };
    }

    fn get_tops(&self) -> String {
        self.stacks.iter().map(|c| c.iter().collect::<String>().trim().chars().last().unwrap_or(' ')).collect::<String>()
    }
}

fn parse_capture(captures: &Captures, name: &str) -> Result<usize, util::Error> {
    captures.name(name)
        .ok_or_else(|| util::Error::new(&format!("Regex could not capture '{name}'")))?
        .as_str()
        .parse::<usize>()
        .map_err(|_| util::Error::new(&format!("Failed to parse '{name}' into usize")))
}

fn parse_line(re: &Regex, line: &str) -> Result<MoveCommand, util::Error> {
    if let Some(cap) = re.captures(line) {
        let amount = parse_capture(&cap, "amount")?;
        let from = parse_capture(&cap, "from")?;
        let to = parse_capture(&cap, "to")?;

        Ok(MoveCommand { amount, from: from - 1, to: to - 1})
    } else {
        Err(util::Error::new("Regex found no captures"))
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<String, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut beginning_state_lines = vec![];
    let mut data: Option<Data> = None;
    let re = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.trim().is_empty() {
            data = Some(Data::parse(&mut beginning_state_lines));
            continue;
        }

        if let Some(ref mut data) = &mut data {
            let cmd = parse_line(&re, line)?;
            data.mov(cmd, Part::One);
        } else {
            beginning_state_lines.push(line.chars().skip(1).step_by(4));
        }
    }

    Ok(data.unwrap().get_tops())
}

pub fn part2<'a, I, S>(lines: I) -> Result<String, util::Error>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut beginning_state_lines = vec![];
    let mut data: Option<Data> = None;
    let re = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for line in lines.into_iter().map(|l| l.as_ref()) {
        if line.trim().is_empty() {
            data = Some(Data::parse(&mut beginning_state_lines));
            continue;
        }

        if let Some(ref mut data) = &mut data {
            let cmd = parse_line(&re, line)?;
            data.mov(cmd, Part::Two);
        } else {
            beginning_state_lines.push(line.chars().skip(1).step_by(4));
        }
    }

    Ok(data.unwrap().get_tops())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2"
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, "MCD");
    }
}
