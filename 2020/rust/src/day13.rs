// Extended euclidean algorithm for computing Bezout's coefficients
fn euclid(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        let temp = a;
        a = b;
        b = temp;
    }

    let (mut s0, mut s1) = (1, 0);
    loop {
        if b == 0 {
            return s0;
        }

        let q = a / b;
        let rem = a % b;
        a = b;
        b = rem;

        // Only s is needed
        let s2 = s0 - q * s1;
        s0 = s1;
        s1 = s2;
    }
}

pub fn part1<'a, I, S>(lines: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter();
    let time = iter.next()?.as_ref().parse::<u32>().ok()?;
    iter.next()?
        .as_ref()
        .split(',')
        .filter_map(|c| c.parse::<u32>().ok())
        .map(|n| (n, (time as f32 / n as f32).ceil() as u32 * n - time))
        .min_by(|x, y| x.1.cmp(&y.1))
        .map(|(bus, time)| bus * time)
}

pub fn part2<'a, I, S>(lines: I) -> Option<i64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter().skip(1);
    let mut ids = iter
        .next()?
        .as_ref()
        .split(',')
        .enumerate()
        .filter_map(|(i, n)| Some((i, n.parse::<i64>().ok()?)))
        .collect::<Vec<_>>();
    ids.sort_unstable_by(|x, y| y.1.cmp(&x.1));

    let mod_product = ids.iter().fold(1, |acc, (_, n)| acc * n);

    let res = ids.iter().fold(0, |acc, &(a, modulo)| {
        // modulos are guaranteed to be coprimes, meaning gcd(m1, m2) = 1
        let m = mod_product / modulo;
        let s = euclid(modulo, m);
        acc + a as i64 * s * m
    }) % mod_product;

    if res < 0 {
        Some(-res)
    } else {
        Some(mod_product - res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["939", "7,13,x,x,59,x,31,19"];
    static EXAMPLE_2: &[&str] = &["0", "17,x,13,19"];
    static EXAMPLE_3: &[&str] = &["0", "67,7,59,61"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 295);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 1068781);
    }

    #[test]
    fn part2_test2() {
        let result = part2(EXAMPLE_2).unwrap();

        assert_eq!(result, 3417);
    }

    #[test]
    fn part2_test3() {
        let result = part2(EXAMPLE_3).unwrap();

        assert_eq!(result, 754018);
    }
}
