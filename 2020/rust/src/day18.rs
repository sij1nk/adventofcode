fn eval<'a, S>(s: S) -> Option<u64>
where
    S: AsRef<str> + 'a,
{
    let line = s.as_ref();
    let mut level = 0;
    let mut parens_expr = String::new();
    let mut acc = 0;
    let mut op = '+';
    for c in line.chars() {
        match c {
            ' ' => continue,
            '+' | '*' => {
                if level == 0 {
                    op = c;
                } else {
                    parens_expr.push(c);
                }
            }
            '(' => {
                if level > 0 {
                    parens_expr.push(c);
                }
                level += 1;
            }
            ')' => {
                level -= 1;
                if level > 0 {
                    parens_expr.push(c);
                } else {
                    if let Some(num) = eval(&parens_expr) {
                        match op {
                            '+' => acc += num,
                            '*' => acc *= num,
                            _ => (),
                        }
                    }
                    parens_expr.clear();
                }
            }
            _ => {
                if level == 0 {
                    let num = c.to_digit(10)? as u64;
                    match op {
                        '+' => acc += num,
                        '*' => acc *= num,
                        _ => (),
                    }
                } else {
                    parens_expr.push(c);
                }
            }
        }
    }

    Some(acc)
}

fn eval2<'a, S>(s: S) -> Option<u64>
where
    S: AsRef<str> + 'a,
{
    let line = s.as_ref();
    let mut nums = vec![];
    let mut level = 0;
    let mut parens_expr = String::new();
    let mut acc = 0;
    for c in line.chars() {
        match c {
            ' ' | '+' => continue,
            '*' => {
                if level == 0 {
                    nums.push(acc);
                    acc = 0;
                } else {
                    parens_expr.push(c);
                }
            }
            '(' => {
                if level > 0 {
                    parens_expr.push(c);
                }
                level += 1;
            }
            ')' => {
                level -= 1;
                if level > 0 {
                    parens_expr.push(c);
                } else {
                    if let Some(num) = eval2(&parens_expr) {
                        acc += num;
                    }
                    parens_expr.clear();
                }
            }
            _ => {
                if level == 0 {
                    acc += c.to_digit(10)? as u64;
                } else {
                    parens_expr.push(c);
                }
            }
        }
    }
    nums.push(acc);

    Some(nums.iter().product())
}

pub fn part1<'a, I, S>(lines: I) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;
    for line in lines.into_iter() {
        sum += eval(line)?;
    }

    Some(sum)
}

pub fn part2<'a, I, S>(lines: I) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut sum = 0;
    for line in lines.into_iter() {
        sum += eval2(line)?;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(51, part1(&["1 + (2 * 3) + (4 * (5 + 6))"]).unwrap());
        assert_eq!(26, part1(&["2 * 3 + (4 * 5)"]).unwrap());
        assert_eq!(437, part1(&["5 + (8 * 3 + 9 + 3 * 4 * 3)"]).unwrap());
        assert_eq!(
            12240,
            part1(&["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"]).unwrap()
        );
        assert_eq!(
            13632,
            part1(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"]).unwrap()
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(51, part2(&["1 + (2 * 3) + (4 * (5 + 6))"]).unwrap());
        assert_eq!(46, part2(&["2 * 3 + (4 * 5)"]).unwrap());
        assert_eq!(1445, part2(&["5 + (8 * 3 + 9 + 3 * 4 * 3)"]).unwrap());
        assert_eq!(
            669060,
            part2(&["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"]).unwrap()
        );
        assert_eq!(
            23340,
            part2(&["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"]).unwrap()
        );
    }
}
