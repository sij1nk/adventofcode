use std::collections::HashSet;

type World = HashSet<(i32, i32)>;

fn parse_world<'a, I, S>(lines: I) -> anyhow::Result<World>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut papers = HashSet::new();

    for (y, line) in lines.into_iter().map(|l| l.as_ref()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '@' {
                continue;
            }

            papers.insert((x as i32, y as i32));
        }
    }

    Ok(papers)
}

fn get_neighbors_count(world: &World, x: i32, y: i32) -> usize {
    let dxdy = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    dxdy.iter()
        .filter_map(|(dx, dy)| {
            let xx = x + dx;
            let yy = y + dy;

            world.get(&(xx, yy))
        })
        .count()
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let world = parse_world(lines)?;
    let mut count = 0;

    for (x, y) in world.iter() {
        if get_neighbors_count(&world, *x, *y) < 4 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut world = parse_world(lines)?;
    let mut removed = 0;

    loop {
        let mut to_remove = HashSet::new();

        for (x, y) in world.iter() {
            if get_neighbors_count(&world, *x, *y) < 4 {
                to_remove.insert((*x, *y));
            }
        }

        if to_remove.is_empty() {
            break;
        }

        removed += to_remove.len();

        world = &world - &to_remove;
    }

    Ok(removed as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 43);
    }
}
