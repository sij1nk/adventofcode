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
    checked: bool,
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
            checked: false,
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

#[derive(Debug, Default)]
struct Window2 {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    current_line: u32,
}

impl Window2 {
    fn add_line(&mut self, y: u32, line: &str) {
        let (mut numbers, mut symbols) = parse_line(y, line);
        self.current_line = y;
        self.numbers.retain(|n| y - n.y <= 1);
        self.symbols.retain(|s| y - s.position.y <= 1);

        self.numbers.append(&mut numbers);
        self.symbols.append(&mut symbols);
    }

    fn process(&mut self) -> u32 {
        let mut sum = 0;

        for number in self.numbers.iter_mut().filter(|n| !n.checked) {
            if self.symbols.iter().any(|s| number.is_adjacent_to(s)) {
                sum += number.value;
                number.checked = true;
            }
        }

        sum
    }
}

fn parse_line(y: u32, line: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut last_position = Position::new(0, y);
    let mut parsed_number = ParsedNumber::default();

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

    if parsed_number.has_digits() {
        numbers.push(parsed_number.finalize());
    }

    (numbers, symbols)
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;

    let mut window = Window2::default();

    for (y, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        window.add_line(y as u32, line);
        sum += window.process();
    }

    Ok(sum)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        let (mut line_numbers, mut line_symbols) = parse_line(y as u32, line);
        numbers.append(&mut line_numbers);
        symbols.append(&mut line_symbols);
    }

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
