use build_const::build_const;
use std::collections::HashSet;
use std::env;

#[rustfmt::skip]
use aoc2023::{day1, day2, day3, day4, day5, day6, /*%main.rs_import%*/};

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
    if args.is_empty() || args.contains("3") {
        println!("Day 3");
        println!("{:?}", day3::part1(DAY3)?);
        println!("{:?}", day3::part2(DAY3)?);
    }
    if args.is_empty() || args.contains("4") {
        println!("Day 4");
        println!("{:?}", day4::part1(DAY4)?);
        println!("{:?}", day4::part2(DAY4)?);
    }
    if args.is_empty() || args.contains("5") {
        println!("Day 5");
        println!("{:?}", day5::part1(DAY5)?);
        println!("{:?}", day5::part2(DAY5)?);
    }
    if args.is_empty() || args.contains("6") {
        println!("Day 6");
        println!("{:?}", day6::part1(DAY6)?);
        println!("{:?}", day6::part2(DAY6)?);
    }
    /*%main.rs_call%*/

    Ok(())
}
