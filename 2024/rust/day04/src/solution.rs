// (x, y), starting from north, clockwise
const DIRECTION_DELTAS: &[(i32, i32)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn build_grid<'a, I, S>(lines: I) -> (Vec<Vec<char>>, i32, i32)
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let grid = lines
        .into_iter()
        .map(|s| s.as_ref())
        .map(|l| l.bytes().map(char::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    (grid, grid_height, grid_width)
}

fn spells_xmas_count(
    grid: &[Vec<char>],
    grid_height: i32,
    grid_width: i32,
    x_y: i32,
    x_x: i32,
) -> u32 {
    DIRECTION_DELTAS
        .iter()
        .map(|&(xd, yd)| {
            let s_y = x_y + 3 * yd;
            let s_x = x_x + 3 * xd;

            let out_of_bounds = s_y < 0 || s_y >= grid_height || s_x < 0 || s_x >= grid_width;
            if out_of_bounds {
                return 0;
            }

            let m_y = (x_y + yd) as usize;
            let m_x = (x_x + xd) as usize;
            let a_y = (x_y + 2 * yd) as usize;
            let a_x = (x_x + 2 * xd) as usize;
            let s_y = s_y as usize;
            let s_x = s_x as usize;

            let spells_xmas =
                grid[m_y][m_x] == 'M' && grid[a_y][a_x] == 'A' && grid[s_y][s_x] == 'S';
            if spells_xmas {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (grid, grid_height, grid_width) = build_grid(lines);

    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &letter) in row.iter().enumerate() {
            let y = y as i32;
            let x = x as i32;

            if letter != 'X' {
                continue;
            }

            count += spells_xmas_count(grid.as_slice(), grid_height, grid_width, y, x);
        }
    }

    Ok(count)
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 18);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 0);
    }
}
