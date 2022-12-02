use std::error;
use std::fmt;

enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct SignParseError;

impl fmt::Display for SignParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrong sign format; expected one of: A, B, C, X, Y, Z")
    }
}

impl error::Error for SignParseError {}

#[derive(Debug)]
enum RoundParseError {
    WrongSign,
    WrongLineFormat,
}

impl fmt::Display for RoundParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::WrongSign => write!(f, "Wrong sign format; expected one of: A, B, C, X, Y, Z"),
            Self::WrongLineFormat => write!(
                f,
                "Wrong line format; expected two Signs separated by a space"
            ),
        }
    }
}

impl error::Error for RoundParseError {}

#[derive(PartialEq, Copy, Clone)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn get_counter(&self) -> Self {
        match self {
            Sign::Rock => Sign::Paper,
            Sign::Paper => Sign::Scissors,
            Sign::Scissors => Sign::Rock,
        }
    }

    fn get_countee(&self) -> Self {
        match self {
            Sign::Rock => Sign::Scissors,
            Sign::Paper => Sign::Rock,
            Sign::Scissors => Sign::Paper,
        }
    }

    fn get_sign_points(&self) -> u32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }

    fn try_parse(value: char, other_sign: Option<&Sign>) -> Result<Self, SignParseError> {
        if let Some(other_sign) = other_sign {
            match value {
                'X' => Ok(other_sign.get_countee()),
                'Y' => Ok(*other_sign),
                'Z' => Ok(other_sign.get_counter()),
                _ => Err(SignParseError {}),
            }
        } else {
            match value {
                'A' | 'X' => Ok(Sign::Rock),
                'B' | 'Y' => Ok(Sign::Paper),
                'C' | 'Z' => Ok(Sign::Scissors),
                _ => Err(SignParseError {}),
            }
        }
    }
}

struct Round {
    my_sign: Sign,
    other_sign: Sign,
}

impl Round {
    fn new(my_sign: Sign, other_sign: Sign) -> Self {
        Self {
            my_sign,
            other_sign,
        }
    }

    fn play(&self) -> u32 {
        let points = self.my_sign.get_sign_points();
        if self.my_sign == self.other_sign {
            points + 3
        } else if self.my_sign == self.other_sign.get_counter() {
            points + 6
        } else {
            points
        }
    }

    fn try_parse(value: &str, part: Part) -> Result<Self, RoundParseError> {
        let (other_sign_str, my_sign_str) = value
            .split_once(' ')
            .ok_or(RoundParseError::WrongLineFormat)?;

        let other_sign_char = other_sign_str
            .chars()
            .next()
            .ok_or(RoundParseError::WrongSign {})?;

        let my_sign_char = my_sign_str
            .chars()
            .next()
            .ok_or(RoundParseError::WrongSign {})?;

        let other_sign =
            Sign::try_parse(other_sign_char, None).map_err(|_| RoundParseError::WrongSign {})?;

        let my_sign = match part {
            Part::One => {
                Sign::try_parse(my_sign_char, None).map_err(|_| RoundParseError::WrongSign {})?
            }
            Part::Two => Sign::try_parse(my_sign_char, Some(&other_sign))
                .map_err(|_| RoundParseError::WrongSign {})?,
        };

        Ok(Round::new(my_sign, other_sign))
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<u32, Box<dyn error::Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut total_points = 0;
    for line in lines.into_iter().map(|l| l.as_ref()) {
        let round = Round::try_parse(line, Part::One)?;
        total_points += round.play();
    }

    Ok(total_points)
}

pub fn part2<'a, I, S>(lines: I) -> Result<u32, Box<dyn error::Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut total_points = 0;
    for line in lines.into_iter().map(|l| l.as_ref()) {
        let round = Round::try_parse(line, Part::Two)?;
        total_points += round.play();
    }

    Ok(total_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["A Y", "B X", "C Z"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 15);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 12);
    }
}
