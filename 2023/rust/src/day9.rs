fn get_differences(numbers: &[i32]) -> Vec<i32> {
    let mut differences = Vec::with_capacity(numbers.len() - 1);

    let mut previous = None;
    for number in numbers {
        if let Some(p) = previous {
            differences.push(number - p);
        }
        previous = Some(number);
    }

    differences
}

fn get_next_value_in_sequence(numbers: &[i32]) -> i32 {
    let differences = get_differences(numbers);

    let last = numbers.last().expect("input list should not be empty");
    if differences.iter().all(|n| *n == differences[0]) {
        last + differences[0]
    } else {
        last + get_next_value_in_sequence(&differences)
    }
}

fn get_previous_value_in_sequence(numbers: &[i32]) -> i32 {
    let differences = get_differences(numbers);

    let first = numbers.first().expect("input list should not be empty");
    if differences.iter().all(|n| *n == differences[0]) {
        first - differences[0]
    } else {
        first - get_previous_value_in_sequence(&differences)
    }
}

fn parse_numbers(line: &str) -> anyhow::Result<Vec<i32>> {
    Ok(line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<i32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let numbers = parse_numbers(line)?;
        sum += get_next_value_in_sequence(&numbers)
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<i32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let numbers = parse_numbers(line)?;
        sum += get_previous_value_in_sequence(&numbers)
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 114);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 2);
    }
}
