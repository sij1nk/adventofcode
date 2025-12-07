use anyhow::anyhow;

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut sum = 0;

    let mut lines = lines.into_iter().map(|l| l.as_ref());

    let first_line = lines.next().ok_or_else(|| anyhow!("Input is missing"))?;
    for word in first_line.split_whitespace() {
        let num = word.parse::<u64>()?;
        nums.push(vec![num]);
    }

    for line in lines {
        let mut words = line.split_whitespace().enumerate().peekable();
        // probably premature optimization; this benchmarks almost the same as if I checked on
        // every word
        let &(_, first_word) = words.peek().ok_or_else(|| anyhow!("Line is empty"))?;

        if first_word == "*" || first_word == "+" {
            for (i, word) in words {
                if word == "*" {
                    sum += nums[i].iter().product::<u64>();
                } else {
                    sum += nums[i].iter().sum::<u64>();
                }
            }
        } else {
            for (i, word) in words {
                let num = word.parse::<u64>()?;
                nums[i].push(num);
            }
        }
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut input: Vec<Vec<char>> = vec![];

    let mut lines = lines.into_iter().map(|l| l.as_ref());

    let first_line_chars = lines
        .next()
        .ok_or_else(|| anyhow!("Input is missing"))?
        .chars();
    for char in first_line_chars {
        input.push(vec![char]);
    }

    for line in lines {
        for (i, char) in line.chars().enumerate() {
            input[i].push(char);
        }
    }

    let mut sum = 0;
    let mut nums = vec![];
    let mut op = ' ';

    for line in input.into_iter() {
        let empty = line.iter().all(|&c| c == ' ');
        if empty {
            if op == '+' {
                sum += nums.iter().sum::<u64>();
            } else if op == '*' {
                sum += nums.iter().product::<u64>();
            }
            nums.clear();
            continue;
        }

        let maybe_op = line.last().cloned().expect("Line is not empty");
        if maybe_op == '*' || maybe_op == '+' {
            op = maybe_op;
        }

        let num: u64 = line
            .iter()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .map(|(i, d)| (d * 10u32.pow(i as u32)) as u64)
            .sum();
        nums.push(num);
    }

    // last transposed line is not empty, so we need to do this one more time
    if op == '+' {
        sum += nums.iter().sum::<u64>();
    } else if op == '*' {
        sum += nums.iter().product::<u64>();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 4277556);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 3263827);
    }
}
