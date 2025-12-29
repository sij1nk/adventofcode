use std::collections::{BTreeSet, VecDeque};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Lights(u16);

impl Lights {
    fn parse(s: &str) -> Self {
        Self(
            s.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '.' => 0,
                    '#' => 1,
                    die => panic!("unexpected char in lights: '{}'", die),
                } * 2_i32.pow(i as u32) as u16)
                .sum(),
        )
    }

    fn solve(&self, buttons: &[Button]) -> u32 {
        let mut queue = VecDeque::from([(0, 0)]);
        let mut seen = BTreeSet::<u16>::new();

        loop {
            let (value, cost) = queue.pop_front().expect("queue to have elements");
            for Button(b) in buttons.iter() {
                let new = value ^ b;
                if new == self.0 {
                    return cost + 1;
                }
                if !seen.contains(&new) {
                    seen.insert(new);
                    queue.push_back((new, cost + 1));
                }
            }
        }
    }
}

impl Display for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
struct Button(u16);

impl Button {
    fn parse(s: &str) -> Self {
        Self(
            s.split(",")
                .map(|n| {
                    2_i32.pow(
                        n.parse::<u32>().expect(
                            "button definition to contain only numbers separated by commas",
                        ),
                    ) as u16
                })
                .sum(),
        )
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (lights, rest) = line.split_once("]").expect("input to contain ']'");
        let lights = &lights[1..]; // remove '['
        let lights = Lights::parse(lights);

        let (buttons, _) = rest.split_once("{").expect("input to contain '{'");
        let buttons = buttons
            .split("(")
            .skip(1)
            .map(|b| {
                let (button, _) = b.split_once(")").expect("buttons to be enclosed in parens");
                Button::parse(button)
            })
            .collect::<Vec<_>>();

        sum += lights.solve(&buttons);
    }

    Ok(sum)
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

    static EXAMPLE: &[&str] = &[
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 7);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
