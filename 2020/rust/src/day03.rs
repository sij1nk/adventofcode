fn count_trees<'a, I, S>(lines: I, right: usize, down: usize) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut count = 0;
    let mut pos = 0;
    let line_width = 31;
    for line in lines.into_iter().step_by(down) {
        let chars: Vec<char> = line.as_ref().chars().collect();
        if chars[pos] == '#' {
            count += 1;
        }
        pos = (pos + right) % line_width;
    }

    count
}

pub fn part1<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    count_trees(lines, 3, 1)
}

pub fn part2<'a, I, S>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a S> + Clone,
    S: AsRef<str> + 'a,
{
    count_trees(lines.clone(), 1, 1)
        * count_trees(lines.clone(), 3, 1)
        * count_trees(lines.clone(), 5, 1)
        * count_trees(lines.clone(), 7, 1)
        * count_trees(lines.clone(), 1, 2)
}
