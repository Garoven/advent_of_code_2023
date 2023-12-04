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

fn main() -> advent_of_code_2023::Result<()> {
    let args = Args::parse();
    let input = &std::fs::read_to_string(args.input)?;

    macro_rules! print_day {
        ($day:tt) => {
            println!(
                "Part one: {:?}\nPart two: {:?}",
                $day::part_1(input),
                $day::part_2(input)
            )
        };
    }

    let ts = std::time::SystemTime::now();
    match args.day {
        1 => print_day!(day_01),
        2 => print_day!(day_02),
        3 => print_day!(day_03),
        4 => print_day!(day_04),
        5..=25 => unimplemented!(),
        _ => unreachable!(),
    };
    println!("Elapsed: {:?}", ts.elapsed());

    Ok(())
}
