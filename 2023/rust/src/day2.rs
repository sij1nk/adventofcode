use anyhow::anyhow;
use std::cmp;

#[derive(Copy, Clone, Debug, Default)]
struct Cubes {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl Cubes {
    fn add_cubes(&mut self, color: &str, amount: u32) -> anyhow::Result<()> {
        match color {
            "red" => self.red_cubes += amount,
            "green" => self.green_cubes += amount,
            "blue" => self.blue_cubes += amount,
            unknown => return Err(anyhow!("Invalid input line - unknown color {}", unknown)),
        }

        Ok(())
    }

    fn maximize_from_pull(&mut self, pull: &Cubes) {
        self.red_cubes = cmp::max(self.red_cubes, pull.red_cubes);
        self.green_cubes = cmp::max(self.green_cubes, pull.green_cubes);
        self.blue_cubes = cmp::max(self.blue_cubes, pull.blue_cubes);
    }

    fn passes_first_game_rule(&self) -> bool {
        self.red_cubes <= 12 && self.green_cubes <= 13 && self.blue_cubes <= 14
    }

    fn power(&self) -> u32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

fn parse_pull(pull_str: &str) -> anyhow::Result<Cubes> {
    let mut pull = Cubes::default();

    for cubes in pull_str.split(',').map(|s| s.trim()) {
        let (amount_str, color) = cubes.split_once(' ').ok_or(anyhow!(
            "Invalid input line - failed to parse pull amount and color"
        ))?;
        let amount = amount_str.parse::<u32>()?;
        pull.add_cubes(color, amount)?;
    }

    Ok(pull)
}

fn parse_game_data(game_data: &str) -> anyhow::Result<Cubes> {
    let mut cubes = Cubes::default();

    for pull_str in game_data.split(';').map(|s| s.trim()) {
        let pull = parse_pull(pull_str)?;
        cubes.maximize_from_pull(&pull);
    }

    Ok(cubes)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (game_prefix, game_data) = line.split_once(':').ok_or(anyhow!(
            "Invalid input line - couldn't find game prefix string"
        ))?;
        let (_, game_identifier_str) = game_prefix.split_once(' ').ok_or(anyhow!(
            "Invalid input line - couldn't find game identifier string"
        ))?;
        let game_identifier = game_identifier_str.parse::<u32>()?;

        let cubes = parse_game_data(game_data)?;

        if cubes.passes_first_game_rule() {
            sum += game_identifier;
        }
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;

    for line in lines.into_iter().map(|l| l.as_ref()) {
        let (_, game_data) = line.split_once(':').ok_or(anyhow!(
            "Invalid input line - couldn't find game prefix string"
        ))?;

        let cubes = parse_game_data(game_data)?;
        sum += cubes.power();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 2286);
    }
}
