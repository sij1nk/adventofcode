#[derive(Debug, Clone, Copy, Default)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn is_adjacent_to(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    value: char,
    position: Position,
}

impl Symbol {
    fn new(value: char, position: Position) -> Self {
        Self { value, position }
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    y: u32,
    x_start: u32,
    x_end: u32,
}

impl Number {
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        self.y.abs_diff(symbol.position.y) <= 1
            && (self.x_start.checked_sub(1).unwrap_or(0)..=self.x_end + 1)
                .contains(&symbol.position.x)
    }
}

#[derive(Debug, Default)]
struct ParsedNumber {
    digits: Vec<(u32, Position)>,
}

impl ParsedNumber {
    fn add_digit(&mut self, digit: u32, position: Position) {
        self.digits.push((digit, position));
    }

    fn has_digits(&self) -> bool {
        !self.digits.is_empty()
    }

    fn finalize(&mut self) -> Number {
        let mut number = Number {
            value: 0,
            y: self.digits.first().unwrap().1.y,
            x_start: self.digits.first().unwrap().1.x,
            x_end: self.digits.last().unwrap().1.x,
        };

        for (index, &(digit, _)) in self.digits.iter().rev().enumerate() {
            let index = index as u32;
            let value = digit * u32::pow(10, index);
            number.value += value;
        }

        self.digits = vec![];

        number
    }
}

fn parse_input<'a, I, S>(lines: I) -> anyhow::Result<(Vec<Number>, Vec<Symbol>)>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut last_position = Position::default();
    let mut parsed_number = ParsedNumber::default();

    for (y, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        for (x, char) in line.chars().enumerate().filter(|&c| c.1 != '.') {
            let position = Position::new(x as u32, y as u32);

            if let Some(digit) = char.to_digit(10) {
                if parsed_number.has_digits() {
                    if !position.is_adjacent_to(&last_position) {
                        numbers.push(parsed_number.finalize());
                    }
                }
                parsed_number.add_digit(digit, position);
            } else {
                symbols.push(Symbol::new(char, position));
                if parsed_number.has_digits() {
                    numbers.push(parsed_number.finalize());
                }
            }

            last_position = position;
        }
    }

    if parsed_number.has_digits() {
        numbers.push(parsed_number.finalize());
    }

    Ok((numbers, symbols))
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;
    let (numbers, symbols) = parse_input(lines)?;
    for number in numbers.into_iter() {
        if symbols.iter().any(|s| number.is_adjacent_to(s)) {
            sum += number.value;
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
    let (numbers, symbols) = parse_input(lines)?;

    for symbol in symbols.into_iter().filter(|s| s.value == '*') {
        let adjacent_number_values: Vec<u32> = numbers
            .iter()
            .filter(|n| n.is_adjacent_to(&symbol))
            .map(|n| n.value)
            .collect();
        if adjacent_number_values.len() == 2 {
            sum += adjacent_number_values.iter().product::<u32>();
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 467835);
    }
}
