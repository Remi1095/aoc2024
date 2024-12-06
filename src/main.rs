use std::{collections::HashMap, error::Error};

use clap::{builder::RangedI64ValueParser, value_parser, Parser, Subcommand};

const FIRST_DAY: i64 = 1;
const LAST_DAY: i64 = 25;
const CURRENT_DAY: i64 = 1;

type AnyError = Box<dyn Error>;
type Func = Box<dyn Fn()>;

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
    },
    All,
}

fn main() -> Result<(), AnyError> {
    let cli = Cli::parse();

    let solution_runners: HashMap<u32, Box<dyn Fn()>> = HashMap::new();

    match cli.command {
        Command::Run { day } => {
            let runner = solution_runners
                .get(&day.unwrap_or(CURRENT_DAY as u32))
                .ok_or(format!("Day {day:?} not implemented yet"))?;
            runner();
        }
        Command::All => {
            let mut solution_runners = solution_runners.into_iter().collect::<Vec<(u32, Func)>>();
            solution_runners.sort_by_key(|(day, _)| *day);
            for (day, runner) in solution_runners {
                print!("Day {}", day);
                runner();
            }
        },
    }

    Ok(())
}

fn day_parser() -> RangedI64ValueParser<u32> {
    value_parser!(u32).range(FIRST_DAY..=LAST_DAY)
}
