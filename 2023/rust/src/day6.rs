use std::iter::zip;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn new(time: i64, distance: i64) -> Self {
        Self { time, distance }
    }

    // For better performance, we binary search for the lowest
    // time T, for which T * (maxtime - T) > distance
    fn get_possible_win_count(&self) -> i64 {
        let adjust_step = |race: &Race, c: i64, s: i64| {
            if c * (race.time - c) > race.distance {
                (s / 2, -1)
            } else {
                (s / 2, 1)
            }
        };

        let mut current = self.time / 2;
        let (mut step, mut direction) = adjust_step(self, current, self.time / 2);

        while step.abs() >= 1 {
            current += step * direction;
            (step, direction) = adjust_step(self, current, step);
        }

        let mut total = ((self.time / 2) - current) * 2;
        if direction == -1 {
            total += 2
        };
        if self.time % 2 == 0 {
            total - 1
        } else {
            total
        }
    }
}

fn parse_races<'a, I, S>(lines: I) -> Vec<Race>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut number_lines = lines.into_iter().map(|l| {
        l.as_ref()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect("could not parse number"))
    });

    let times = number_lines.next().expect("could not parse times line");
    let distances = number_lines.next().expect("could not parse distances line");

    zip(times, distances)
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn parse_races_with_very_bad_keming<'a, I, S>(lines: I) -> Race
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut number_lines = lines.into_iter().map(|l| {
        l.as_ref()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .replace(' ', "")
            .parse::<i64>()
            .expect("could not parse number")
    });

    let time = number_lines.next().expect("could not parse times line");
    let distance = number_lines.next().expect("could not parse distances line");

    Race::new(time, distance)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<i64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let races = parse_races(lines);
    Ok(races.iter().map(|r| r.get_possible_win_count()).product())
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<i64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let race = parse_races_with_very_bad_keming(lines);
    Ok(race.get_possible_win_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["Time:      7  15   30", "Distance:  9  40  200"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 71503);
    }
}
