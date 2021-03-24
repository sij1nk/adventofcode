pub fn part1<'a, I, S>(lines: I) -> u16
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter().map(|l| l.as_ref());
    let mut valid_ranges = vec![];
    let mut parsed_ranges = Vec::with_capacity(40);
    let mut sum = 0;
    loop {
        if let Some(line) = iter.next() {
            if line == "" {
                break;
            }

            for split in line.rsplitn(4, ' ').step_by(2).take(2) {
                let mut nums = split.split('-');
                if let Some(Ok(bot)) = nums.next().map(|v| v.parse::<u16>()) {
                    if let Some(Ok(top)) = nums.next().map(|v| v.parse::<u16>()) {
                        parsed_ranges.push((bot, top));
                    }
                }
            }
        }
    }

    while let Some((mut new_bot, mut new_top)) = parsed_ranges.pop() {
        valid_ranges = valid_ranges
            .iter()
            .filter_map(|&(b, t)| {
                if t < new_bot - 1 || b > new_top + 1 {
                    Some((b, t))
                } else {
                    new_bot = new_bot.min(b);
                    new_top = new_top.max(t);
                    None
                }
            })
            .collect();
        valid_ranges.push((new_bot, new_top));
    }

    println!("{:?}", valid_ranges);

    let mut iter = iter.skip(4);

    while let Some(line) = iter.next() {
        for num in line.split(',').filter_map(|n| n.parse::<u16>().ok()) {
            if !valid_ranges
                .iter()
                .any(|&(bot, top)| num >= bot && num <= top)
            {
                sum += num;
                break;
            }
        }
    }

    sum
}

pub fn part2<'a, I, S>(lines: I) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter().map(|l| l.as_ref());
    let mut columns: Vec<Vec<u16>> = vec![vec![]; 20];
    let mut ranges = Vec::with_capacity(20);
    let mut valid_ranges = vec![];
    let mut combinations = vec![vec![]; 20];

    // Collect ranges from first 1/3 of input
    loop {
        if let Some(line) = iter.next() {
            if line == "" {
                break;
            }

            let mut rs = line
                .rsplitn(4, ' ')
                .step_by(2)
                .take(2)
                .map(|s| s.split('-').filter_map(|v| v.parse::<u16>().ok()));
            if let Some(mut i) = rs.next() {
                let n1 = i.next()?;
                let n2 = i.next()?;

                if let Some(mut i) = rs.next() {
                    let n3 = i.next()?;
                    let n4 = i.next()?;
                    ranges.push((n3, n4, n1, n2));
                }
            };
        }
    }

    // Determine the range of valid numbers
    for &(bot1, top1, bot2, top2) in ranges.iter() {
        for &(mut bot, mut top) in [(bot1, top1), (bot2, top2)].iter() {
            valid_ranges = valid_ranges
                .iter()
                .filter_map(|&(b, t)| {
                    if t < bot - 1 || b > top + 1 {
                        Some((b, t))
                    } else {
                        bot = bot.min(b);
                        top = top.max(t);
                        None
                    }
                })
                .collect();
            valid_ranges.push((bot, top));
        }
    }

    // Collect second 1/3 of input that contains my ticket
    let mut iter = iter.skip(1);
    let my_nums = iter
        .next()?
        .split(',')
        .filter_map(|v| v.parse::<u16>().ok())
        .collect::<Vec<_>>();

    // Collect last 1/3 of input and group them into columns
    let mut iter = iter.skip(2);
    while let Some(line) = iter.next() {
        let nums = line
            .split(',')
            .filter_map(|v| v.parse::<u16>().ok())
            .collect::<Vec<_>>();
        // Discard tickets that contain invalid numbers
        if nums
            .iter()
            .all(|&n| valid_ranges.iter().any(|&(bot, top)| bot <= n && n <= top))
        {
            let mut i = 0;
            for num in nums.iter() {
                columns[i].push(*num);
                i += 1;
            }
        }
    }

    // Determine possible column - range group combinations
    for (i_col, col) in columns.iter().enumerate() {
        for (i_range, (bot1, top1, bot2, top2)) in ranges.iter().enumerate() {
            if col
                .iter()
                .all(|n| (bot1 <= n && n <= top1) || (bot2 <= n && n <= top2))
            {
                combinations[i_col].push(i_range);
            }
        }
    }

    let mut combinations = combinations
        .into_iter()
        .enumerate()
        .map(|(i, p)| (i, p))
        .collect::<Vec<_>>();
    combinations.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut pairings: Vec<(usize, usize)> = vec![];
    for comb in combinations.into_iter() {
        let new_pairing = (
            comb.0,
            *comb
                .1
                .iter()
                .filter(|&&c| pairings.iter().all(|&p| p.1 != c))
                .next()?,
        );
        pairings.push(new_pairing);
    }

    Some(
        pairings
            .into_iter()
            .filter_map(|(c, r)| if r < 6 { Some(c) } else { None })
            .map(|i| my_nums[i] as u64)
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "seat: 13-40 or 45-50",
        "",
        "your ticket:",
        "7,1,14",
        "",
        "nearby tickets:",
        "7,3,47",
        "40,4,50",
        "55,2,20",
        "38,6,12",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE);

        assert_eq!(result, 71);
    }
}
