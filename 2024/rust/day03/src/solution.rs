pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

pub fn part2<'a, I, S>(_lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] =
        &["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 161);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
