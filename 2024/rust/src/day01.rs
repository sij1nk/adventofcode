use std::collections::BTreeMap;

fn parse_number_maybe(word: Option<&str>) -> anyhow::Result<u32> {
    word.ok_or(anyhow::anyhow!("word is None"))
        .and_then(|word| word.parse::<u32>().map_err(anyhow::Error::msg))
}

fn build_occurrence_map(numbers: &mut [u32]) -> anyhow::Result<BTreeMap<u32, u32>> {
    numbers.sort();

    let mut map = BTreeMap::<u32, u32>::new();

    let mut current = numbers
        .first()
        .ok_or(anyhow::anyhow!("numbers shouldn't be empty"))?;
    let mut current_occurrences = 0;

    for n in numbers.iter() {
        if n == current {
            current_occurrences += 1;
        } else {
            map.insert(*current, current_occurrences);
            current = n;
            current_occurrences = 1;
        }
    }

    map.insert(*current, current_occurrences);

    Ok(map)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut n1s = Vec::new();
    let mut n2s = Vec::new();
    for line in lines.into_iter().map(|item| item.as_ref()) {
        let mut words = line.split_whitespace();
        let n1 = parse_number_maybe(words.next())?;
        let n2 = parse_number_maybe(words.next())?;
        n1s.push(n1);
        n2s.push(n2);
    }

    n1s.sort();
    n2s.sort();

    let result = n1s
        .iter()
        .zip(n2s.iter())
        .map(|(&n1, &n2)| n1.abs_diff(n2))
        .sum::<u32>();

    Ok(result)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut n1s = Vec::new();
    let mut n2s = Vec::new();
    for line in lines.into_iter().map(|item| item.as_ref()) {
        let mut words = line.split_whitespace();
        let n1 = parse_number_maybe(words.next())?;
        let n2 = parse_number_maybe(words.next())?;
        n1s.push(n1);
        n2s.push(n2);
    }

    n1s.sort();
    let occurrence_map = build_occurrence_map(&mut n2s)?;

    let result = n1s
        .into_iter()
        .map(|n| occurrence_map.get(&n).unwrap_or(&0) * n)
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 11);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 31);
    }
}
