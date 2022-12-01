use build_const::build_const;
use std::collections::HashSet;
use std::env;
use std::io;

use aoc2022::{day01, util};

build_const!("aoc2022");

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 01");
        println!(
            "{:?}",
            day01::part1(DAY1)
                .ok_or_else(|| util::to_ioerror(util::Error))? // Unpack Option
        );
        println!(
            "{:?}",
            day01::part2(DAY1)
                .ok_or_else(|| util::to_ioerror(util::Error))? // Unpack Result
        );
    }

    /*%CALL%*/

    Ok(())
}
