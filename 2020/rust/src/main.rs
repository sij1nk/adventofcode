use build_const::build_const;
use std::collections::HashSet;
use std::env;
use std::io;

use aoc2020::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, util,
};

build_const!("aoc2020");

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<String>>();

    if args.is_empty() || args.contains("1") {
        println!("Day 01");
        println!(
            "{:?}",
            day01::part1(DAY1)
                .map_err(util::to_ioerror)? // Unpack Result
                .ok_or_else(|| util::to_ioerror(util::Error))? // Unpack Option
        );
        println!(
            "{:?}",
            day01::part2(DAY1)
                .map_err(util::to_ioerror)?
                .ok_or_else(|| util::to_ioerror(util::Error))?
        );
    }

    if args.is_empty() || args.contains("2") {
        println!("Day 02");
        println!("{:?}", day02::part1(DAY2));
        println!("{:?}", day02::part2(DAY2));
    }

    if args.is_empty() || args.contains("3") {
        println!("Day 03");
        println!("{:?}", day03::part1(DAY3));
        println!("{:?}", day03::part2(DAY3));
    }

    if args.is_empty() || args.contains("4") {
        println!("Day 04");
        println!("{:?}", day04::part1(DAY4));
        println!("{:?}", day04::part2(DAY4));
    }

    if args.is_empty() || args.contains("5") {
        println!("Day 05");
        println!(
            "{:?}",
            day05::part1(DAY5).ok_or_else(|| util::to_ioerror(util::Error))?
        );
        println!(
            "{:?}",
            day05::part2(DAY5).ok_or_else(|| util::to_ioerror(util::Error))?
        );
    }

    if args.is_empty() || args.contains("6") {
        println!("Day 06");
        println!("{:?}", day06::part1(DAY6));
        println!("{:?}", day06::part2(DAY6));
    }

    if args.is_empty() || args.contains("7") {
        println!("Day 07");
        println!("{:?}", day07::part1(DAY7));
        println!("{:?}", day07::part2(DAY7));
    }

    if args.is_empty() || args.contains("8") {
        println!("Day 08");
        println!("{:?}", day08::part1(DAY8).map_err(util::to_ioerror)?);
        println!("{:?}", day08::part2(DAY8).map_err(util::to_ioerror)?);
    }

    if args.is_empty() || args.contains("9") {
        println!("Day 09");
        println!(
            "{:?}",
            day09::part1(DAY9, 25).ok_or_else(|| util::to_ioerror(util::Error))?
        );
        println!(
            "{:?}",
            day09::part2(DAY9, 25).ok_or_else(|| util::to_ioerror(util::Error))?
        );
    }

    if args.is_empty() || args.contains("10") {
        println!("Day 10");
        println!("{:?}", day10::part1(DAY10).map_err(util::to_ioerror)?);
        println!("{:?}", day10::part2(DAY10).map_err(util::to_ioerror)?);
    }

    if args.is_empty() || args.contains("11") {
        println!("Day 11");
        println!("{:?}", day11::part1(DAY11));
        println!("{:?}", day11::part2(DAY11));
    }

    if args.is_empty() || args.contains("12") {
        println!("Day 12");
        println!("{:?}", day12::part1(DAY12).map_err(util::to_ioerror)?);
        println!("{:?}", day12::part2(DAY12).map_err(util::to_ioerror)?);
    }
    Ok(())
}
