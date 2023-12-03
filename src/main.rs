use clap::Parser;
use std::fmt::Debug;

use advent_of_code_2023::*;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    day: u8,
}

macro_rules! print_day {
    ($day:tt, $i:expr) => {
        println!(
            "Part one: {:?}\nPart two: {:?}",
            $day::part_1($i),
            $day::part_2($i)
        )
    };
}

fn main() -> advent_of_code_2023::Result<()> {
    let args = Args::parse();
    let input = &std::fs::read_to_string(args.input)?;

    match args.day {
        1 => print_day!(day_01, input),
        2 => print_day!(day_02, input),
        3 => print_day!(day_03, input),
        4..=25 => unimplemented!(),
        _ => unreachable!(),
    };

    Ok(())
}
