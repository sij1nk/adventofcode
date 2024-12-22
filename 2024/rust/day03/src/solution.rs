use itertools::Itertools;
use regex::Regex;

fn parse_u64(input: &str) -> anyhow::Result<u64> {
    input.parse::<u64>().map_err(|e| e.into())
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let input =
        Itertools::intersperse(lines.into_iter().map(|s| s.as_ref()), " ").collect::<String>();

    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let captures_iter = regex.captures_iter(&input);

    captures_iter
        .map(|caps| {
            let n1 = parse_u64(&caps[1])?;
            let n2 = parse_u64(&caps[2])?;

            Ok(n1 * n2)
        })
        .sum()
}

#[derive(Debug)]
struct Instruction {
    location: usize,
    data: InstructionData,
}

#[derive(Debug, Clone, Copy)]
enum InstructionData {
    Do,
    Dont,
    Mul(u64, u64),
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let input =
        Itertools::intersperse(lines.into_iter().map(|s| s.as_ref()), " ").collect::<String>();

    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut instructions = Vec::<Instruction>::new();

    let mul_instructions = mul_regex.captures_iter(&input).filter_map(|caps| {
        let m = caps.get(0).unwrap();
        let n1 = parse_u64(&caps[1]).ok()?;
        let n2 = parse_u64(&caps[2]).ok()?;

        Some(Instruction {
            location: m.start(),
            data: InstructionData::Mul(n1, n2),
        })
    });

    let do_instructions = do_regex.find_iter(&input).map(|m| Instruction {
        location: m.start(),
        data: InstructionData::Do,
    });

    let dont_instructions = dont_regex.find_iter(&input).map(|m| Instruction {
        location: m.start(),
        data: InstructionData::Dont,
    });

    instructions.extend(mul_instructions);
    instructions.extend(do_instructions);
    instructions.extend(dont_instructions);

    instructions.sort_by(|i1, i2| i1.location.cmp(&i2.location));

    let mut sum: u64 = 0;
    let mut enabled = true;

    for ins in instructions.into_iter() {
        match ins.data {
            InstructionData::Do => enabled = true,
            InstructionData::Dont => enabled = false,
            InstructionData::Mul(n1, n2) => {
                if enabled {
                    sum += n1 * n2
                }
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] =
        &["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 161);
    }

    static EXAMPLE2: &[&str] =
        &["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE2).unwrap();

        assert_eq!(result, 48);
    }
}
