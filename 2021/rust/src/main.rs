use build_const::build_const;
use std::collections::HashSet;
use std::env;
use std::io;

use aoc2021::{day01, /*%IMPORT%*/ util};

build_const!("aoc2021");

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
                .map_err(util::to_ioerror)? // Unpack Result
        );
    }

    /*%CALL%*/

    Ok(())
}
