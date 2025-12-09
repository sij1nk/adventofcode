use day08::{read_input, solution};

fn main() -> anyhow::Result<()> {
    let input = read_input()?;

    println!("Day 08");
    println!("{:?}", solution::part1(&input, 1000));
    println!("{:?}", solution::part2(&input));

    Ok(())
}
