use anyhow::anyhow;

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let nums = line
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("Failed to parse numbers"))?;

        let len = nums.len();

        let (i, first) = nums[..len - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .ok_or_else(|| anyhow!("Iterator was empty"))?;

        let second = nums[i + 1..]
            .iter()
            .max()
            .ok_or_else(|| anyhow!("Iterator was empty"))?;

        let n = format!("{first}{second}").parse::<u32>()?;

        sum += n;
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let nums = line
            .chars()
            .map(|c| c.to_digit(10).map(|n| n as u64))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("Failed to parse numbers"))?;

        let len = nums.len();

        let mut digits = vec![];

        let mut min_i = 0;

        for d in (0..12).rev() {
            let (i, digit) = nums[min_i..len - d]
                .iter()
                .enumerate()
                .rev()
                .max_by(|(_, x), (_, y)| x.cmp(y))
                .ok_or_else(|| anyhow!("Iterator was empty"))?;

            digits.push(digit);

            min_i += i + 1;
        }

        let n: u64 = digits
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| *n * 10u64.pow(i as u32))
            .sum();

        sum += n;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 357);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 3121910778619);
    }
}
