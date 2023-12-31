use advent_of_code_2023::*;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    day: u8,
}

fn main() -> advent_of_code_2023::Result<()> {
    let args: Args = clap::Parser::parse();
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
        5 => print_day!(day_05),
        6 => print_day!(day_06),
        7 => print_day!(day_07),
        8 => print_day!(day_08),
        9 => print_day!(day_09),
        10 => print_day!(day_10),
        11 => print_day!(day_11),
        12 => print_day!(day_12),
        13 => print_day!(day_13),
        14 => print_day!(day_14),
        15 => print_day!(day_15),
        16 => print_day!(day_16),
        17..=25 => unimplemented!(),
        _ => unreachable!(),
    };
    println!("Elapsed: {:?}", ts.elapsed());

    Ok(())
}
