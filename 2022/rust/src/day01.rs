pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut max = 0;
    let mut current = 0;
    for line in lines.into_iter().map(|item| item.as_ref()) {
        if line.is_empty() {
            if max < current {
                max = current;
            }

            current = 0;
        } else {
            let num = line.parse::<u32>().unwrap();
            current += num;
        }
    }

    // If last line isn't a blank line
    if max < current {
        max = current;
    }

    Some(max)
}

fn get_new_maxes((mut max0, mut max1, mut max2): (u32, u32, u32), current: u32) -> (u32, u32, u32) {
    if current > max2 {
        if current > max1 {
            if current > max0 {
                max2 = max1;
                max1 = max0;
                max0 = current;
            } else {
                max2 = max1;
                max1 = current;
            }
        } else {
            max2 = current;
        }
    }

    (max0, max1, max2)
}

pub fn part2<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (mut max0, mut max1, mut max2) = (0, 0, 0);
    let mut current = 0;

    for line in lines.into_iter().map(|item| item.as_ref()) {
        if line.is_empty() {
            (max0, max1, max2) = get_new_maxes((max0, max1, max2), current);
            current = 0;
        } else {
            let num = line.parse::<u32>().unwrap();
            current += num;
        }
    }

    // If last line isn't a blank line
    (max0, max1, max2) = get_new_maxes((max0, max1, max2), current);

    Some(max0 + max1 + max2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 45000);
    }
}
