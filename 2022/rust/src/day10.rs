use anyhow::anyhow;

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<i32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut register: i32 = 1;
    let mut cycle_count: i32 = 0;
    const INTERESTING_CYCLES: &[i32] = &[20, 60, 100, 140, 180, 220];

    let mut sum: i32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        // Last line of output may be empty, let's get around that
        if line.is_empty() {
            continue;
        }

        let mut words = line.split(' ');

        let Some(opcode) = words.next() else {
            return Err(anyhow!("Line did not contain opcode"));
        };

        let next_cycle_count = match opcode {
            "addx" => cycle_count + 2,
            "noop" => cycle_count + 1,
            _ => return Err(anyhow!("Unknown opcode")),
        };

        for interesting_cycle in INTERESTING_CYCLES {
            let current_cycle_range = cycle_count + 1..=next_cycle_count;
            if current_cycle_range.contains(interesting_cycle) {
                let num = interesting_cycle * register;
                sum += num;
            }
        }

        match opcode {
            "addx" => {
                let arg = words
                    .next()
                    .ok_or(anyhow!("Line did not contain addx argument"))?
                    .parse::<i32>()
                    .map_err(|_| anyhow!("Could not parse addx argument"))?;

                register += arg;
            }
            "noop" => {}
            _ => return Err(anyhow!("Unknown opcode")),
        }

        cycle_count = next_cycle_count
    }

    Ok(sum)
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

    static EXAMPLE: &[&str] = &[
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
