use day01::{read_input, solution};

fn main() -> anyhow::Result<()> {
    // TODO:
    // - automate setting up new days
    //   - pkg template
    //   - day placeholders in pkg
    //   - add pkg to workspace Cargo.toml
    //   - download input for day
    let input = read_input()?;

    println!("Day 01");
    println!("{:?}", solution::part1(&input));
    println!("{:?}", solution::part2(&input));

    Ok(())
}
