use aoc2024::util;
use build_const::build_const;
use std::collections::HashSet;
use std::env;
use std::io;

use aoc2024::{day01, day02};

build_const!("aoc2024");

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 01");
        println!("{:?}", day01::part1(DAY1).map_err(util::to_ioerror));
        println!("{:?}", day01::part2(DAY1).map_err(util::to_ioerror));
    }

    if args.is_empty() || args.contains("2") {
        println!("Day 02");
        println!("{:?}", day02::part1(DAY2).map_err(util::to_ioerror));
        println!("{:?}", day02::part2(DAY2).map_err(util::to_ioerror));
    }

    Ok(())
}
