pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut depth: u32 = 0;
    let mut position: u32 = 0;
    for line in lines
        .into_iter()
        .map(|line| line.as_ref())
        .filter(|line| !line.is_empty())
    {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u32>().unwrap();

        match command {
            "forward" => {
                position += amount;
            }
            "down" => {
                depth += amount;
            }
            "up" => {
                depth -= amount;
            }
            _ => unreachable!(),
        }
    }

    Some(position * depth)
}

pub fn part2<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut aim: u32 = 0;
    let mut depth: u32 = 0;
    let mut position: u32 = 0;
    for line in lines
        .into_iter()
        .map(|line| line.as_ref())
        .filter(|line| !line.is_empty())
    {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u32>().unwrap();

        match command {
            "forward" => {
                position += amount;
                depth += amount * aim;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => unreachable!(),
        }
    }

    Some(position * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 150);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 900);
    }
}
