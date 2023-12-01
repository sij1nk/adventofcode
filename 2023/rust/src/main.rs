use build_const::build_const;
use std::collections::HashSet;
use std::env;

use aoc2023::day01;

build_const!("aoc2023");

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 01");
        println!("{:?}", day01::part1(DAY1)?);
        println!("{:?}", day01::part2(DAY1)?);
    }

    Ok(())
}
