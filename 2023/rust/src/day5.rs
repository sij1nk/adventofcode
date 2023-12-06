use std::cmp;
use std::collections::BTreeMap;

type Diff = i64;

// std::ops::Range is overkill; we don't need to iterate over it,
// we only need the start and the end. This way we can also make
// it Copy
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: &i64) -> bool {
        self.start <= *value && *value < self.end
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let start_ordering = self.start.cmp(&other.start);
        if start_ordering == cmp::Ordering::Equal {
            other.end.cmp(&self.end)
        } else {
            start_ordering
        }
    }
}

fn simplify_ranges(ranges: &mut [Range]) -> Vec<Range> {
    let mut simplified_ranges = Vec::new();
    ranges.sort();

    let mut iter = ranges.iter();
    let first = *iter.next().expect("input vec of ranges is empty");
    let last_merged = iter.fold(first, |r1, &r2| {
        if r1.end < r2.start {
            simplified_ranges.push(r1);
            r2
        } else {
            Range::new(cmp::min(r1.start, r2.start), cmp::max(r1.end, r2.end))
        }
    });

    simplified_ranges.push(last_merged);

    simplified_ranges
}

#[derive(Debug, Default)]
struct TranslationMap {
    name: String,
    inner: BTreeMap<Range, Diff>,
}

impl TranslationMap {
    fn translate_number_inplace(&self, number: &mut i64) {
        let mut new_value = *number;
        for (range, diff) in self.inner.iter() {
            if range.contains(number) {
                new_value += diff;
                break;
            }
            if range.start > *number {
                new_value = *number;
                break;
            }
        }

        *number = new_value;
    }

    fn translate_range(&self, range: Range) -> Vec<Range> {
        let mut ranges = Vec::new();

        let mut leftover = range;
        for (i, (translation_range, diff)) in self.inner.iter().enumerate() {
            // tr. o---o . . . . . .
            // lo.        o---o
            //            ^^^^^
            if translation_range.end <= leftover.start {
                if i == self.inner.len() - 1 {
                    ranges.push(leftover);
                    break;
                } else {
                    continue;
                }
            }

            // tr. . . . . . o---o
            // lo. o---o
            //     ^^^^^
            if leftover.end <= translation_range.start {
                ranges.push(leftover);
                break;
            }

            let range_start = if translation_range.start < leftover.start {
                leftover.start
            } else {
                translation_range.start
            };
            let range_end = if translation_range.end > leftover.end {
                leftover.end
            } else {
                translation_range.end
            };

            // tr. . . o---o
            // lo.   o---o
            //       ^^^
            if leftover.start < translation_range.start {
                ranges.push(Range::new(leftover.start, translation_range.start));
            }

            // tr. . . o---o   OR . . . o---o
            // lo.   o---o                o----o
            //         ^^^                ^^^
            ranges.push(Range::new(range_start + diff, range_end + diff));

            // tr. . o---o
            // lo.     o----o
            //           ****
            if leftover.end > translation_range.end {
                leftover = Range::new(translation_range.end, leftover.end);
            } else {
                break;
            }
        }

        ranges
    }
}

fn parse_seeds_part1(line: &str) -> Vec<i64> {
    line.split_once(':')
        .map(|(_, numbers)| {
            numbers
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("cannot parse seed"))
                .collect::<Vec<_>>()
        })
        .expect("cannot parse seeds")
}

fn parse_seeds_part2(line: &str) -> Vec<Range> {
    let mut ranges = Vec::new();

    let numbers = line
        .split_once(':')
        .map(|(_, numbers)| {
            numbers
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect("cannot parse seed"))
                .collect::<Vec<_>>()
        })
        .expect("cannot parse seeds");

    for chunk in numbers.chunks(2) {
        ranges.push(Range::new(chunk[0], chunk[0] + chunk[1]));
    }

    ranges
}

fn parse_maps<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<TranslationMap> {
    let mut maps: Vec<TranslationMap> = Vec::new();
    let mut current_map = TranslationMap::default();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if !current_map.inner.is_empty() {
                maps.push(current_map);
                current_map = TranslationMap::default();
                current_map.name = line.into();
            }
            continue;
        }

        let mut map_description = line.split_whitespace().map(|n| {
            n.parse::<i64>()
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
            .insert(Range::new(source, source + range), dest - source);
    }

    if !current_map.inner.is_empty() {
        maps.push(current_map);
    }

    maps
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<i64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut lines = lines.into_iter().map(|l| l.as_ref());
    let seeds_line = lines.next().expect("input is empty");

    let mut seeds = parse_seeds_part1(seeds_line);
    let maps = parse_maps(lines);

    for map in maps.into_iter() {
        for seed in seeds.iter_mut() {
            map.translate_number_inplace(seed);
        }
    }

    let lowest_location = seeds.iter().min().expect("could not find lowest location");

    Ok(*lowest_location)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<i64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut lines = lines.into_iter().map(|l| l.as_ref());
    let seeds_line = lines.next().expect("input is empty");

    let mut seed_ranges = parse_seeds_part2(seeds_line);
    let maps = parse_maps(lines);

    for map in maps.into_iter() {
        let mut new_seed_ranges: Vec<Range> = Vec::with_capacity(seed_ranges.len());
        for seed_range in seed_ranges.iter() {
            new_seed_ranges.append(&mut map.translate_range(*seed_range));
        }
        seed_ranges = simplify_ranges(&mut new_seed_ranges);
    }

    let lowest_location = seed_ranges
        .iter()
        .min()
        .expect("could not find lowest location")
        .start;

    Ok(lowest_location)
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

        assert_eq!(result, 46);
    }

    #[test]
    fn translate_range_works() {
        let translation_map = TranslationMap {
            name: "Test".into(),
            inner: BTreeMap::from([
                (Range::new(0, 10), 10),
                (Range::new(10, 15), 20),
                (Range::new(15, 20), -15),
                (Range::new(30, 50), 5),
            ]),
        };

        assert_eq!(
            translation_map.translate_range(Range::new(0, 10)),
            vec![Range::new(10, 20)]
        );

        assert_eq!(
            translation_map.translate_range(Range::new(0, 25)),
            vec![
                Range::new(10, 20),
                Range::new(30, 35),
                Range::new(0, 5),
                Range::new(20, 25)
            ]
        );

        let translation_map2 = TranslationMap {
            name: "Test2".into(),
            inner: BTreeMap::from([(Range::new(50, 98), 2), (Range::new(98, 100), -48)]),
        };

        assert_eq!(
            translation_map2.translate_range(Range::new(14, 79)),
            vec![Range::new(14, 50), Range::new(52, 81)]
        );

        assert_eq!(
            translation_map2.translate_range(Range::new(13, 55)),
            vec![Range::new(13, 50), Range::new(52, 57)]
        );

        let translation_map3 = TranslationMap {
            name: "Test3".into(),
            inner: BTreeMap::from([(Range::new(52, 54), -15)]),
        };

        assert_eq!(
            translation_map3.translate_range(Range::new(57, 70)),
            vec![Range::new(57, 70)]
        );
    }

    #[test]
    fn simplify_ranges_works() {
        assert_eq!(
            simplify_ranges(&mut vec![Range::new(14, 79), Range::new(13, 55)]),
            vec![Range::new(13, 79)]
        );

        assert_eq!(
            simplify_ranges(&mut vec![
                Range::new(14, 50),
                Range::new(52, 81),
                Range::new(13, 50),
                Range::new(52, 57)
            ]),
            vec![Range::new(13, 50), Range::new(52, 81)]
        );
    }
}
