fn parse_seats<'a, I, S>(lines: I) -> Vec<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut seats = Vec::new();
    for line in lines.into_iter() {
        let mut row = 0;
        let mut col = 0;

        for (i, bit) in line.as_ref().chars().rev().enumerate() {
            if i < 3 {
                if bit == 'R' {
                    col += 2_usize.pow(i as u32);
                }
            } else {
                if bit == 'B' {
                    row += 2_usize.pow((i - 3) as u32);
                }
            }
        }

        seats.push(row * 8 + col);
    }

    seats
}

pub fn part1<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let seats = parse_seats(lines);

    seats.into_iter().max()
}

pub fn part2<'a, I, S>(lines: I) -> Option<usize>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut seats = parse_seats(lines);
    seats.sort();

    let mut prev = seats.first().unwrap();
    for cur in seats.iter().skip(1) {
        if prev + 1 != *cur {
            return Some(*cur - 1);
        }
        prev = cur;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

    #[test]
    fn parse_test() {
        let results = parse_seats(EXAMPLE);
        assert_eq!(results, [357, 567, 119, 820]);
    }
}
