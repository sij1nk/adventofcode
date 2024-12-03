use day03::{read_input, solution};

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Day 03");
    println!("{:?}", solution::part1(&input));
    println!("{:?}", solution::part2(&input));

    Ok(())
}
