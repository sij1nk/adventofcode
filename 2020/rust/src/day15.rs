fn van_eck<'a, I, S>(lines: I, goal: u32) -> u32
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut last_seen = vec![0; goal as usize];
    let mut last: u32 = 0;
    let mut t = 1;

    for line in lines.into_iter() {
        let line = line.as_ref();
        let chars: Vec<_> = line.split(',').collect();
        println!("{:?}", chars);
        for n in chars.iter().take(chars.len() - 1) {
            let n = n.parse::<u32>().unwrap();
            last_seen[n as usize] = t;
            t += 1;
        }
        last = chars[chars.len() - 1].parse::<u32>().unwrap();
    }

    println!("{:?}", last_seen);
    println!("{}", last);

    loop {
        if t == goal {
            return last;
        }

        let old_t = last_seen[last as usize];
        last_seen[last as usize] = t;
        if old_t == 0 {
            last = 0;
        } else {
            last = t - old_t;
        }

        t += 1;
    }
}

pub fn part1<'a, I, S>(lines: I) -> u32
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    van_eck(lines, 2020)
}

pub fn part2<'a, I, S>(lines: I) -> u32
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    van_eck(lines, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["0,3,6"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);

        assert_eq!(result, 436);
    }
}
