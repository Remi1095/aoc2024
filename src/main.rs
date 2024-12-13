use aoc2024::{solution_runners, Runner};
use clap::{builder::RangedI64ValueParser, value_parser, Parser, Subcommand};
use std::error::Error;

pub const FIRST_DAY: i64 = 1;
pub const LAST_DAY: i64 = 25;
pub const FIRST_PART: i64 = 1;
pub const LAST_PART: i64 = 2;

pub type AnyError = Box<dyn Error>;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(
            short,
            long,
            value_parser = day_parser(),
        )]
        day: Option<u32>,
        #[arg(
            short,
            long,
            value_parser = part_parser(),
        )]
        part: Option<u32>,
    },
    All,
}

fn main() -> Result<(), AnyError> {
    let cli = Cli::parse();

    let solution_runners = solution_runners();

    let selected_runners = match cli.command {
        Command::Run { day, mut part } => {


            let day_or_max = day.unwrap_or(
                *solution_runners
                    .keys()
                    .reduce(|max_day, day: &u32| if day > max_day { day } else { max_day })
                    .ok_or("No day implemented")?,
            );
            let runners = solution_runners
                .get(&day_or_max)
                .ok_or(format!("Day {day_or_max:?} not implemented"))?;

            if day == None && part == None{
                part = Some(runners.len() as u32);
            }

            if let Some(part) = part {
                let runner = runners
                    .get(part as usize - 1)
                    .ok_or("Part does not exist")?;
                vec![(day_or_max, part, runner)]
            } else {
                runners
                    .iter()
                    .enumerate()
                    .map(|(idx, runner)| (day_or_max, idx as u32 + 1, runner))
                    .collect()
            }
        }
        Command::All => {
            let mut selected: Vec<(u32, u32, &Runner)> = Vec::new();
            for (day, runners) in solution_runners.iter() {
                for (part, runner) in runners.iter().enumerate() {
                    selected.push((*day, part as u32 + 1, runner));
                }
            }
            selected.sort_by_key(|(day, part, _)| (*day, *part));
            selected
        }
    };
    for (day, part, runner) in selected_runners {
        println!("Day {} part {}", day, part);
        let solution = runner()?;
        println!("Solution: {}\n", solution);
    }

    Ok(())
}

fn day_parser() -> RangedI64ValueParser<u32> {
    value_parser!(u32).range(FIRST_DAY..=LAST_DAY)
}

fn part_parser() -> RangedI64ValueParser<u32> {
    value_parser!(u32).range(1..=10)
}
