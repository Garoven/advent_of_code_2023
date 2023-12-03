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

    let (a, b): (Box<dyn Debug>, Box<dyn Debug>) = match args.day {
        1 => (
            Box::new(day_01::part_1(input)),
            Box::new(day_01::part_2(input)),
        ),
        2 => (
            Box::new(day_02::part_1(input)),
            Box::new(day_02::part_2(input)),
        ),
        3 => (
            Box::new(day_03::part_1(input)),
            Box::new(day_03::part_2(input)),
        ),
        4..=25 => unimplemented!(),
        _ => unreachable!(),
    };

    println!("Part one: {a:?}\nPart two: {b:?}");

    Ok(())
}
