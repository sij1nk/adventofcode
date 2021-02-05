#[derive(Debug)]
struct Password<'a> {
    num1: usize,
    num2: usize,
    c: char,
    pw: &'a str,
}

fn parse_passwords<'a, I, S>(lines: I) -> Vec<Password<'a>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .filter_map(|l| {
            let tokens: Vec<&str> = l.as_ref().split(' ').collect();
            if let [nums, c, pw] = &tokens[..] {
                let nums = nums
                    .split('-')
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()
                    .ok()?;
                let c = c.chars().next()?;

                return Some(Password {
                    num1: nums[0],
                    num2: nums[1],
                    c,
                    pw,
                });
            }

            None
        })
        .collect()
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let passwords = parse_passwords(lines);
    passwords
        .into_iter()
        .filter_map(|p| {
            let n = p.pw.chars().filter(|&c| c == p.c).count();
            if n < p.num1 || n > p.num2 {
                return None;
            }
            Some(p)
        })
        .count()
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let passwords = parse_passwords(lines);
    passwords
        .into_iter()
        .filter_map(|p| {
            let chars: Vec<char> = p.pw.chars().collect();
            if (chars[p.num1 - 1] == p.c) != (chars[p.num2 - 1] == p.c) {
                return Some(p);
            }

            None
        })
        .count()
}
