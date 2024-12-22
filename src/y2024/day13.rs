use itertools::Itertools;
use num::{rational::Ratio, traits::SaturatingSub, CheckedSub, Rational};
use regex::Regex;
use std::{
    f32,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{get_text_file, math::Vec2, SolutionResult};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/13/input";

struct ClawMachine {
    button_a: Vec2<u32>,
    button_b: Vec2<u32>,
    prize: Vec2<u32>,
}

const BUTTON_A_COST: u32 = 3;
const BUTTON_B_COST: u32 = 1;

const PRIZE_OFFSET: u64 = 10_000_000_000_000;

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let tokens = iter_input(file)
        .filter_map(
            |ClawMachine {
                 button_a: a,
                 button_b: b,
                 prize,
             }| {
                // println!("\na {:?} b {:?}", a, b);
                let a_x_ratio = Ratio::new(prize.x, a.x);
                let a_y_ratio = Ratio::new(prize.y, a.y);
                // println!(
                //     "a_x_ratio {} a_y_ratio {}",
                //     a_x_ratio.to_integer(),
                //     a_y_ratio.to_integer()
                // );
                let a_cost = (a_x_ratio + a_y_ratio) * BUTTON_A_COST;
                let b_x_ratio = Ratio::new(prize.x, b.x);
                let b_y_ratio = Ratio::new(prize.y, b.y);
                // println!(
                //     "b_x_ratio {} b_y_ratio {}",
                //     b_x_ratio.to_integer(),
                //     b_y_ratio.to_integer()
                // );
                let b_cost = (b_x_ratio + b_y_ratio) * BUTTON_B_COST;
                // println!("a_cost {} b_cost {}", a_cost, b_cost);

                let ((good_step, good_cost), (bad_step, bad_cost)) = if a_cost < b_cost {
                    // println!("a good b bad");
                    ((a, BUTTON_A_COST), (b, BUTTON_B_COST))
                } else {
                    // println!("a bad b good");
                    ((b, BUTTON_B_COST), (a, BUTTON_A_COST))
                };
                // a*n + b*m = prize
                for num_bad_step in 0..=100 {
                    let bad_step_result = bad_step * num_bad_step;
                    if let Some(good_step_result) = prize.checked_sub(&bad_step_result) {
                        let num_good_step = good_step_result.x / good_step.x;
                        if good_step_result.x % good_step.x == 0
                            && good_step_result.y % good_step.y == 0
                            && num_good_step == good_step_result.y / good_step.y
                        {
                            // println!("num_bad_step {} num_good_step {}", num_bad_step, num_good_step);
                            return Some(num_bad_step * bad_cost + num_good_step * good_cost);
                        }
                    } else {
                        break;
                    }
                }
                None
            },
        )
        .sum::<u32>() as i64;

    Ok(tokens)
}

pub fn part_2() -> SolutionResult {
    Ok(0)
}

fn iter_input(file: File) -> impl Iterator<Item = ClawMachine> {
    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok().filter(|line| !line.is_empty()))
        .tuples::<(_, _, _)>()
        .map(move |(line_1, line_2, line_3)| {
            // println!("line_1 {} line_2 {} line_3 {}", line_1, line_2, line_3);
            ClawMachine {
                button_a: parse_vec2(&line_1, &button_regex),
                button_b: parse_vec2(&line_2, &button_regex),
                prize: parse_vec2(&line_3, &prize_regex),
            }
        })
}

fn parse_vec2(line: &str, re: &Regex) -> Vec2<u32> {
    let [x, y] = re
        .captures(line)
        .unwrap()
        .extract()
        .1
        .map(|n| n.parse::<u32>().unwrap());
    Vec2 { x, y }
}
