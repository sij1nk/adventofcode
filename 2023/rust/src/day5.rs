use std::collections::BTreeMap;

use anyhow::anyhow;

type N = i64;
type Source = N;
type Diff = N;
type RangeMax = N;

#[derive(Debug, Default)]
struct TranslationMap {
    inner: BTreeMap<Source, (Diff, RangeMax)>,
}

impl TranslationMap {
    fn transform_number(&self, number: &mut N) {
        let mut new_value = *number;
        for (&source, &(diff, range_max)) in self.inner.iter() {
            if (source..range_max).contains(number) {
                new_value += diff;
                break;
            }
            if source > *number {
                new_value = *number;
                break;
            }
        }

        *number = new_value;
    }
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<N>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut lines = lines.into_iter().map(|l| l.as_ref());
    let mut maps: Vec<TranslationMap> = Vec::new();

    let Some(mut seeds) = lines
        .next()
        .and_then(|l| l.split_once(':'))
        .and_then(|(_, numbers)| {
            Some(
                numbers
                    .split_whitespace()
                    .map(|n| n.parse::<Source>().expect("cannot parse seed"))
                    .collect::<Vec<_>>(),
            )
        })
    else {
        return Err(anyhow!("Was unable to parse seeds"));
    };

    let mut current_map = TranslationMap::default();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if !current_map.inner.is_empty() {
                maps.push(current_map);
                current_map = TranslationMap::default();
            }
            continue;
        }

        let mut map_description = line.split_whitespace().map(|n| {
            n.parse::<N>()
                .expect("cannot parse number in map description")
        });
        let dest = map_description
            .next()
            .expect("cannot parse dest in map description");
        let source = map_description
            .next()
            .expect("cannot parse source in map description");
        let range = map_description
            .next()
            .expect("cannot parse range in map description");

        current_map
            .inner
            .insert(source, (dest - source, range + source));
    }

    if !current_map.inner.is_empty() {
        maps.push(current_map);
    }

    for map in maps.into_iter() {
        for seed in seeds.iter_mut() {
            map.transform_number(seed);
        }
    }

    let lowest_location = seeds.iter().min().expect("could not find lowest location");

    Ok(*lowest_location)
}

pub fn part2<'a, I, S>(_lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 35);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
