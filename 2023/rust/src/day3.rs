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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
struct SlidingWindow {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    current_line_index: u32,
    capacity: u32,
    size: u32,
}

impl SlidingWindow {
    fn new(capacity: u32) -> Self {
        Self {
            numbers: Vec::with_capacity(32),
            symbols: Vec::with_capacity(32),
            current_line_index: 0,
            capacity,
            size: 0,
        }
    }
}

impl SlidingWindow {
    fn add_line(&mut self, line: &str) {
        self.slide_out();

        let (mut numbers, mut symbols) = parse_line(self.current_line_index, line);
        self.numbers.append(&mut numbers);
        self.symbols.append(&mut symbols);
        if self.size < self.capacity {
            self.size += 1;
        }
    }

    fn slide_out(&mut self) {
        self.current_line_index += 1;

        self.numbers
            .retain(|n| self.current_line_index - n.y <= self.capacity - 1);
        self.symbols
            .retain(|s| self.current_line_index - s.position.y <= self.capacity - 1);
    }

    fn process<F>(&mut self, algorithm: F) -> u32
    where
        F: Fn(&mut SlidingWindow) -> u32,
    {
        algorithm(self)
    }
}

fn sum_part_numbers(window: &mut SlidingWindow) -> u32 {
    let mut sum = 0;

    window.numbers.retain(|n| {
        if window.symbols.iter().any(|s| n.is_adjacent_to(s)) {
            sum += n.value;
            false
        } else {
            true
        }
    });

    sum
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;
    let mut window = SlidingWindow::new(2);

    for line in lines.into_iter().map(|l| l.as_ref()) {
        window.add_line(line);
        sum += window.process(sum_part_numbers)
    }

    Ok(sum)
}

fn sum_gear_ratio(window: &mut SlidingWindow) -> u32 {
    let mut sum = 0;

    for symbol in window
        .symbols
        .iter()
        .filter(|s| s.position.y == window.current_line_index - 1)
        .filter(|s| s.value == '*')
    {
        let adjacent_number_values: Vec<u32> = window
            .numbers
            .iter()
            .filter(|n| n.is_adjacent_to(&symbol))
            .map(|n| n.value)
            .collect();
        if adjacent_number_values.len() == 2 {
            sum += adjacent_number_values.iter().product::<u32>();
        }
    }

    sum
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum: u32 = 0;
    let mut window = SlidingWindow::new(3);

    let lines: Vec<_> = lines.into_iter().map(|l| l.as_ref()).collect();
    window.add_line(lines.first().unwrap());

    for line in lines.iter().skip(1) {
        window.add_line(line);
        sum += window.process(sum_gear_ratio);
    }

    window.slide_out();
    sum += window.process(sum_gear_ratio);

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
