fn parse_report(line: &str) -> anyhow::Result<Vec<u32>> {
    let report = line
        .split_whitespace()
        .map(|word| word.parse::<u32>().map_err(anyhow::Error::msg))
        .collect::<anyhow::Result<Vec<u32>>>()?;
    Ok(report)
}

fn is_report_safe(report: &[u32]) -> anyhow::Result<bool> {
    let first_level = report
        .first()
        .ok_or(anyhow::anyhow!("numbers has 0 items"))?;
    let second_level = report
        .get(1)
        .ok_or(anyhow::anyhow!("numbers has only 1 item"))?;

    let is_increasing = first_level < second_level;

    for (n1, n2) in report.iter().zip(report.iter().skip(1)) {
        let are_equal = n1 == n2;
        let wrong_direction = (is_increasing && n1 > n2) || (!is_increasing && n1 < n2);
        let large_diff = n1.abs_diff(*n2) > 3;

        if are_equal || wrong_direction || large_diff {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut safe_count = 0;

    for line in lines.into_iter().map(|item| item.as_ref()) {
        let report = parse_report(line)?;

        let result = is_report_safe(&report);
        if result.is_ok_and(|b| b) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut safe_count = 0;

    for line in lines.into_iter().map(|item| item.as_ref()) {
        let report = parse_report(line)?;

        let result = is_report_safe(&report);
        if result.is_ok_and(|b| b) {
            safe_count += 1;
        } else {
            let mut skip_index = 0;
            while skip_index < report.len() {
                let report_with_skip = report
                    .iter()
                    .enumerate()
                    .filter(|&(i, n)| i != skip_index)
                    .map(|(_i, n)| *n)
                    .collect::<Vec<_>>();
                let result = is_report_safe(&report_with_skip);
                if result.is_ok_and(|b| b) {
                    safe_count += 1;
                    break;
                } else {
                    skip_index += 1;
                }
            }
        }
    }

    Ok(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 4);
    }
}
