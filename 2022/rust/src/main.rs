use build_const::build_const;
use std::collections::HashSet;
use std::env;
use std::io;

use aoc2022::{day01, day02, day03, day04, util};

build_const!("aoc2022");

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 01");
        println!(
            "{:?}",
            day01::part1(DAY1).ok_or_else(|| util::to_ioerror(util::Error::empty()))?
        );
        println!(
            "{:?}",
            day01::part2(DAY1).ok_or_else(|| util::to_ioerror(util::Error::empty()))?
        );
    }

    if args.is_empty() || args.contains("2") {
        println!("Day 02");
        println!("{:?}", day02::part1(DAY2).map_err(util::to_ioerror));
        println!("{:?}", day02::part2(DAY2).map_err(util::to_ioerror));
    }

    if args.is_empty() || args.contains("3") {
        println!("Day 03");
        println!("{:?}", day03::part1(DAY3).ok_or_else(|| util::to_ioerror(util::Error::empty())));
        println!("{:?}", day03::part2(DAY3).ok_or_else(|| util::to_ioerror(util::Error::empty())));
    }

    if args.is_empty() || args.contains("4") {
        println!("Day 04");
        println!("{:?}", day04::part1(DAY4).map_err(util::to_ioerror));
        println!("{:?}", day04::part2(DAY4).map_err(util::to_ioerror));
    }

    Ok(())
}
