type Int = u64;

fn get_digits_count(n: Int) -> Int {
    n.ilog10() as Int + 1
}

fn blink(n: Int) -> Vec<Int> {
    if n == 0 {
        return vec![1];
    };

    let digits_count = get_digits_count(n);
    if digits_count % 2 == 0 {
        let half_digits = digits_count / 2;
        let divisor = (10 as Int).pow(half_digits as u32);

        let last_two = n % divisor;
        let first_two = n / divisor;

        return vec![first_two, last_two];
    }

    vec![n * 2024]
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<Int>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut stones = lines
        .into_iter()
        .map(|s| s.as_ref().split(' ').map(|w| w.parse::<Int>().unwrap()))
        .next()
        .unwrap()
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones.into_iter().flat_map(blink).collect::<Vec<_>>();
    }

    Ok(stones.len() as Int)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<Int>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut stones = lines
        .into_iter()
        .map(|s| s.as_ref().split(' ').map(|w| w.parse::<Int>().unwrap()))
        .next()
        .unwrap()
        .collect::<Vec<_>>();

    for _ in 0..75 {
        let new_stones = stones.into_iter().flat_map(blink).collect::<Vec<_>>();
        stones = new_stones;
    }

    Ok(stones.len() as Int)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["125 17"];

    #[test]
    fn blink_works() {
        let nums: Vec<Int> = vec![0, 1, 2, 99, 2024];

        let result = nums.into_iter().flat_map(blink).collect::<Vec<_>>();

        assert_eq!(result, vec![1, 2024, 4048, 9, 9, 20, 24]);
    }

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 55312);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
