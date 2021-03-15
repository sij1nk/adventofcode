use super::processor::{Instruction, InstructionParseError, Processor};
use std::num::ParseIntError;
use std::str::FromStr;

fn locate_next_swappable(instructions: &[Instruction], idx: Option<usize>) -> Option<usize> {
    if let Some(idx) = idx {
        for (i, ins) in instructions[idx..].iter().enumerate().skip(1) {
            match ins {
                Instruction::Jmp(_) | Instruction::Nop(_) => {
                    return Some(idx + i);
                }
                _ => (),
            }
        }
    }

    None
}

fn swap_next(instructions: &mut [Instruction], idx: Option<usize>, old_idx: Option<usize>) {
    if let Some(old_idx) = old_idx {
        if let Some(ins) = instructions.get_mut(old_idx) {
            if let Instruction::Jmp(n) = ins {
                *ins = Instruction::Nop(*n);
            } else if let Instruction::Nop(n) = ins {
                *ins = Instruction::Jmp(*n);
            }
        }
    }

    if let Some(idx) = idx {
        if let Some(ins) = instructions.get_mut(idx) {
            if let Instruction::Jmp(n) = ins {
                *ins = Instruction::Nop(*n);
            } else if let Instruction::Nop(n) = ins {
                *ins = Instruction::Jmp(*n);
            }
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<i32, InstructionParseError<ParseIntError>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let instructions = lines
        .into_iter()
        .map(|l| Instruction::from_str(l.as_ref()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut processor = Processor::new(instructions.as_slice());
    let mut visited = vec![];
    loop {
        if visited.contains(&processor.ip) {
            return Ok(processor.acc);
        } else {
            visited.push(processor.ip);
            let _ = processor.execute();
        }
    }
}

pub fn part2<'a, I, S>(lines: I) -> Result<i32, InstructionParseError<ParseIntError>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut instructions = lines
        .into_iter()
        .map(|l| Instruction::from_str(l.as_ref()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut swap_idx = Some(0);
    let mut old_swap_idx = None;

    loop {
        swap_next(&mut instructions, swap_idx, old_swap_idx);
        let mut processor = Processor::new(instructions.as_slice());
        let mut visited = vec![];

        loop {
            if visited.contains(&processor.ip) {
                break;
            } else {
                visited.push(processor.ip);
                match processor.execute() {
                    Ok(Some(acc)) => return Ok(acc),
                    Err(_) => break,
                    _ => (),
                }
            }
        }

        old_swap_idx = swap_idx;
        swap_idx = locate_next_swappable(&instructions, swap_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);

        assert_eq!(result, 5);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE);

        assert_eq!(result, 8);
    }
}
