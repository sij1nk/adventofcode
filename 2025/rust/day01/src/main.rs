use day01::{read_input, solution};

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Day 01");
    println!("{:?}", solution::part1(&input));
    println!("{:?}", solution::part2(&input));

    Ok(())
}
