use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    current: u64,
    numbers: Vec<u64>,
}

fn parse_equation(line: impl AsRef<str>) -> Equation {
    let (result_str, numbers_str) = line
        .as_ref()
        .split_once(':')
        .expect("Line should contain a colon");
    let result = result_str.parse::<u64>().expect("Result should be a u64");
    let numbers = numbers_str
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().expect("Numbers should be u64s"))
        .rev()
        .collect::<Vec<_>>();

    Equation {
        result,
        current: result,
        numbers,
    }
}

fn process_equation(eq: Equation, allow_concat: bool) -> Option<u64> {
    let mut sub_equations = VecDeque::from([eq.clone()]);

    while let Some(eq) = sub_equations.pop_front() {
        let mut numbers_iter = eq.numbers.iter();
        let Some(n) = numbers_iter.next() else {
            continue;
        };
        let rest = numbers_iter.copied().collect::<Vec<_>>();

        if eq.current % n == 0 {
            let value = eq.current / n;
            if value == 1 {
                return Some(eq.result);
            }

            sub_equations.push_front(Equation {
                result: eq.result,
                current: value,
                numbers: rest.clone(),
            });
        }

        match eq.current.checked_sub(*n) {
            Some(value) => {
                if allow_concat {
                    let divisor = (10i32.pow(n.ilog10() + 1)) as u64;
                    let last_digit_matches = value % divisor == 0;
                    if last_digit_matches {
                        sub_equations.push_back(Equation {
                            result: eq.result,
                            current: value / divisor,
                            numbers: rest.clone(),
                        })
                    }
                }

                sub_equations.push_back(Equation {
                    result: eq.result,
                    current: value,
                    numbers: rest,
                });
            }
            None => continue,
        }
    }

    None
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(lines
        .into_iter()
        .map(parse_equation)
        .filter_map(|eq| process_equation(eq, false))
        .sum())
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(lines
        .into_iter()
        .map(parse_equation)
        .filter_map(|eq| process_equation(eq, true))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 3749);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 11387);
    }
}
