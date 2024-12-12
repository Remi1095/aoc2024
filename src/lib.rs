pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use reqwest::{blocking, header::COOKIE, Url};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub type AnyError = Box<dyn Error>;
pub type SolutionResult = Result<i32, AnyError>;
pub type Runner = Box<dyn Fn() -> SolutionResult>;

const INPUT_DIR: &str = "input";
const AOC_SESSION_COOKIE_FILE: &str = "aoc_session_cookie.txt";

pub struct Problem {
    pub day: u32,
    pub part: u32,
}

pub fn solution_runners() -> HashMap<u32, Vec<Runner>> {
    let mut solution_runners: HashMap<u32, Vec<Runner>> = HashMap::new();
    let f = |fn_ptr: fn() -> SolutionResult| Box::new(fn_ptr) as Runner;

    solution_runners.extend(
        [
            (1, vec![f(day1::part_1), f(day1::part_2)]),
            (2, vec![f(day2::part_1), f(day2::part_2)]),
            (3, vec![f(day3::part_1), f(day3::part_2)]),
            (4, vec![f(day4::part_1), f(day4::part_2)]),
        ]
        .into_iter(),
    );
    solution_runners
}

pub fn get_text_file(url: &str) -> Result<File, Box<dyn Error>> {
    let directory = INPUT_DIR;
    let parsed_url = Url::parse(url)?;
    let path = parsed_url.path();
    let file_name = path[1..].replace('/', "_");
    let file_name = if file_name.is_empty() {
        "index.txt".to_string()
    } else {
        file_name
    };
    let mut file_path = PathBuf::from(directory);
    file_path.push(file_name);

    if !file_path.exists() {
        let client = blocking::Client::new();
        let mut session_cookie = String::new();
        File::open(AOC_SESSION_COOKIE_FILE)?.read_to_string(&mut session_cookie)?;
        let response_bytes = client
            .get(url)
            .header(COOKIE, format!("session={}", session_cookie))
            .send()?
            .bytes()?;
        fs::create_dir_all(directory)?;
        let mut file = File::create(&file_path)?;
        file.write_all(&response_bytes)?;
    }

    Ok(File::open(&file_path)?)
}
