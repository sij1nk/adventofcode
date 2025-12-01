use %DAY%::{read_input, solution};

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("%DAY_NICE%");
    println!("{:?}", solution::part1(&input));
    println!("{:?}", solution::part2(&input));

    Ok(())
}
