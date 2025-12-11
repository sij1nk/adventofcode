#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V2 {
    x: u64,
    y: u64,
}

impl V2 {
    fn area(&self, other: &V2) -> u64 {
        (self.y.abs_diff(other.y) + 1) * (self.x.abs_diff(other.x) + 1)
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut dots: Vec<V2> = vec![];

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (x_str, y_str) = line.split_once(",").expect("line to contain a comma");
        let x = x_str.parse::<u64>()?;
        let y = y_str.parse::<u64>()?;
        dots.push(V2 { x, y })
    }

    let mut max = 0;

    for (i, d) in dots.iter().enumerate() {
        for dd in dots.iter().skip(i + 1) {
            let area = d.area(dd);
            if area > max {
                max = area;
            }
        }
    }

    Ok(max)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 50);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 24);
    }
}
