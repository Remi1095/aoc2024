use itertools::Itertools;
use num::rational::Ratio;
use regex::Regex;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{get_text_file, math::Vec2, SolutionResult};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/13/input";

struct ClawMachine {
    button_a: Vec2<i64>,
    button_b: Vec2<i64>,
    prize: Vec2<i64>,
}

const BUTTON_A_COST: i64 = 3;
const BUTTON_B_COST: i64 = 1;

const PRIZE_OFFSET: i64 = 10000000000000;
// const PRIZE_OFFSET: i64 = 10_000_000_000;

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let tokens: i64 = iter_input(file)
        .filter_map(
            |ClawMachine {
                 button_a: a,
                 button_b: b,
                 prize,
             }| {
                let a_x_ratio = Ratio::new(prize.x, a.x);
                let a_y_ratio = Ratio::new(prize.y, a.y);
                let a_cost = (a_x_ratio + a_y_ratio) * BUTTON_A_COST;

                let b_x_ratio = Ratio::new(prize.x, b.x);
                let b_y_ratio = Ratio::new(prize.y, b.y);
                let b_cost = (b_x_ratio + b_y_ratio) * BUTTON_B_COST;

                let ((good_step, good_cost), (bad_step, bad_cost)) = if a_cost < b_cost {
                    ((a, BUTTON_A_COST), (b, BUTTON_B_COST))
                } else {
                    ((b, BUTTON_B_COST), (a, BUTTON_A_COST))
                };
                // a*n + b*m = prize
                for num_bad_step in 0..=100 {
                    let bad_step_result = bad_step * num_bad_step;
                    let good_step_result = prize - bad_step_result;
                    if good_step_result.x > 0 && good_step_result.y > 0 {
                        let num_good_step = good_step_result.x / good_step.x;
                        if good_step_result.x % good_step.x == 0
                            && good_step_result.y % good_step.y == 0
                            && num_good_step == good_step_result.y / good_step.y
                        {
                            return Some(num_bad_step * bad_cost + num_good_step * good_cost);
                        }
                    } else {
                        break;
                    }
                }
                None
            },
        )
        .sum();

    Ok(tokens.to_string())
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let tokens: i64 = iter_input(file)
        .filter_map(
            |ClawMachine {
                 button_a: a,
                 button_b: b,
                 prize,
             }| {
                let prize = prize
                    + Vec2 {
                        x: PRIZE_OFFSET,
                        y: PRIZE_OFFSET,
                    };
                let a_x_ratio = Ratio::new(prize.x, a.x);
                let a_y_ratio = Ratio::new(prize.y, a.y);
                let a_cost = (a_x_ratio + a_y_ratio) * BUTTON_A_COST;

                let b_x_ratio = Ratio::new(prize.x, b.x);
                let b_y_ratio = Ratio::new(prize.y, b.y);
                let b_cost = (b_x_ratio + b_y_ratio) * BUTTON_B_COST;

                let ((good_step, good_cost), (bad_step, bad_cost)) = if a_cost < b_cost {
                    ((a, BUTTON_A_COST), (b, BUTTON_B_COST))
                } else {
                    ((b, BUTTON_B_COST), (a, BUTTON_A_COST))
                };
                // a*n + b*m = prize

                // let good_step_result = prize - bad_step * 1;
                let num_good_step_x_diff = Ratio::new(prize.x, good_step.x)
                    - Ratio::new(prize.x - bad_step.x, good_step.x);
                let num_good_step_y_diff = Ratio::new(prize.y, good_step.y)
                    - Ratio::new(prize.y - bad_step.y, good_step.y);
                let order_num_good_step = if num_good_step_x_diff < num_good_step_y_diff {
                    |x, y| (x, y)
                } else {
                    |x, y| (y, x)
                };

                let mut num_bad_step = 0;
                let mut ascending = true;
                let mut min_num_bad_step = 0;
                let mut max_num_bad_step = 0;
                loop {
                    if bad_step.x.checked_mul(num_bad_step).is_none()
                        || bad_step.y.checked_mul(num_bad_step).is_none()
                    {
                        return None;
                    }
                    let good_step_result = prize - bad_step * num_bad_step;

                    let (min_num_good_step, max_num_good_step) = order_num_good_step(
                        Ratio::new(good_step_result.x, good_step.x),
                        Ratio::new(good_step_result.y, good_step.y),
                    );

                    let prev_num_bad_step = num_bad_step;
                    match min_num_good_step.cmp(&max_num_good_step) {
                        Ordering::Less => {
                            if ascending {
                                if num_bad_step == 0 {
                                    num_bad_step = 1;
                                } else {
                                    num_bad_step *= 2;
                                }
                            } else {
                                min_num_bad_step = num_bad_step;
                                num_bad_step += (max_num_bad_step - num_bad_step) / 2;
                                if num_bad_step <= prev_num_bad_step {
                                    return None;
                                }
                            }
                        }
                        Ordering::Greater => {
                            if ascending {
                                ascending = false;
                                min_num_bad_step = num_bad_step / 2;
                                max_num_bad_step = num_bad_step;
                                num_bad_step =
                                    min_num_bad_step + (max_num_bad_step - min_num_bad_step) / 2
                            } else {
                                max_num_bad_step = num_bad_step;
                                num_bad_step -= (num_bad_step - min_num_bad_step) / 2;
                                if num_bad_step >= prev_num_bad_step {
                                    return None;
                                }
                            }
                        }
                        Ordering::Equal => {
                            if min_num_good_step < 0.into() || !min_num_good_step.is_integer() {
                                return None;
                            }
                            return Some(
                                num_bad_step * bad_cost
                                    + min_num_good_step.to_integer() * good_cost,
                            );
                        }
                    }
                }
            },
        )
        .sum();

    Ok(tokens.to_string())
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

fn parse_vec2(line: &str, re: &Regex) -> Vec2<i64> {
    let [x, y] = re
        .captures(line)
        .unwrap()
        .extract()
        .1
        .map(|n| n.parse::<i64>().unwrap());
    Vec2 { x, y }
}
