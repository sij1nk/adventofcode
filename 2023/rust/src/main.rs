use build_const::build_const;
use std::collections::HashSet;
use std::env;

#[rustfmt::skip]
use aoc2023::{day1, day2, /*%main.rs_import%*/};

build_const!("aoc2023");

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 1");
        println!("{:?}", day1::part1(DAY1)?);
        println!("{:?}", day1::part2(DAY1)?);
    }

    if args.is_empty() || args.contains("2") {
        println!("Day 2");
        println!("{:?}", day2::part1(DAY2)?);
        println!("{:?}", day2::part2(DAY2)?);
    }
    /*%main.rs_call%*/

    Ok(())
}
