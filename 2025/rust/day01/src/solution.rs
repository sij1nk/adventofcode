pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut n: i32 = 50;
    let mut zeros = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (dir, d_str) = line.split_at(1);
        let d = d_str.parse::<i32>()?;

        if dir == "L" {
            n = (n - d) % 100;
            if n < 0 {
                n += 100;
            }
        } else if dir == "R" {
            n = (n + d) % 100;
        }

        if n == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut n: i32 = 50;
    let mut zeros = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (dir, d_str) = line.split_at(1);
        let d = d_str.parse::<i32>()?;

        let mut nn = n;

        if dir == "L" {
            nn -= d;
            if nn == 0 {
                zeros += 1;
            }
            if nn < 0 {
                let skip = n == 0; // if we're on 0 and going left, we don't want to count the 0

                zeros += (-nn / 100) + if skip { 0 } else { 1 };
                nn += (-(nn - 99) / 100) * 100;
            }
        } else if dir == "R" {
            nn += d;
            let x = nn / 100;
            zeros += x;
            nn -= x * 100;
        }

        n = nn;
    }

    Ok(zeros as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 6);
    }
}
