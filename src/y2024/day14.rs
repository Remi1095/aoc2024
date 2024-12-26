use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use regex::Regex;

use crate::{get_text_file, math::Vec2, SolutionResult};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/14/input";

const AREA_WIDTH: i64 = 101;
const AREA_HEIGHT: i64 = 103;
// const AREA_WIDTH: i64 = 11;
// const AREA_HEIGHT: i64 = 7;
const ELAPSED: i64 = 100;

#[derive(Clone, Debug)]
struct Robot {
    position: Vec2<i64>,
    velocity: Vec2<i64>,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    let area_width_mid_left = AREA_WIDTH / 2;
    let area_width_mid_right = AREA_WIDTH / 2 + AREA_WIDTH % 2 - 1;
    let area_height_mid_top = AREA_HEIGHT / 2;
    let area_height_mid_botton = AREA_HEIGHT / 2 + AREA_HEIGHT % 2 - 1;
    let robots = read_input(file);

    for Robot { position, velocity } in robots {
        let mut position = position + velocity * ELAPSED;
        position = Vec2 {
            x: position.x % AREA_WIDTH,
            y: position.y % AREA_HEIGHT,
        };
        if position.x < 0 {
            position.x += AREA_WIDTH;
        }
        if position.y < 0 {
            position.y += AREA_HEIGHT;
        }
        let left = position.x < area_width_mid_left;
        let right = position.x > area_width_mid_right;
        let top = position.y < area_height_mid_top;
        let bottom = position.y > area_height_mid_botton;
        if top && left {
            top_left += 1
        } else if top && right {
            top_right += 1;
        } else if bottom && left {
            bottom_left += 1;
        } else if bottom && right {
            bottom_right += 1;
        }
    }
    let safety = top_left * top_right * bottom_left * bottom_right;

    Ok(safety)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let robots = read_input(file);

    for elapsed in 0..10000 {
        let mut positions = robots
            .iter()
            .cloned()
            .map(|Robot { position, velocity }| {
                let mut new_position = position + velocity * elapsed;
                new_position.x %= AREA_WIDTH;
                new_position.y %= AREA_HEIGHT;
                if new_position.x < 0 {
                    new_position.x += AREA_WIDTH;
                }
                if new_position.y < 0 {
                    new_position.y += AREA_HEIGHT;
                }
                new_position
            })
            .collect_vec();
        positions.sort_by(|pos_1, pos_2| match pos_2.y.cmp(&pos_1.y) {
            Ordering::Equal => pos_2.x.cmp(&pos_1.x),
            ord => ord,
        });
        // display_positions(&positions);

        let in_target_width = positions
            .iter()
            .filter(|pos| {
                (AREA_WIDTH / 4..AREA_WIDTH * 3 / 4).contains(&pos.x)
            })
            .count();

        let in_target_height = positions
            .iter()
            .filter(|pos| (AREA_HEIGHT / 4..AREA_HEIGHT * 3 / 4).contains(&pos.y))
            .count();

        let target = robots.len() - robots.len() / 5;

        if in_target_width >= target && in_target_height >= target {
            // println!("robots {} target {}", robots.len(), in_target_width);
            // display_positions(&positions);
            return Ok(elapsed);
        }
    }
    Ok(0)
}

fn read_input(file: File) -> Vec<Robot> {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    BufReader::new(file)
        .lines()
        .map(move |line| {
            let [p_x, p_y, v_x, v_y] = robot_regex
                .captures(&line.unwrap())
                .unwrap()
                .extract()
                .1
                .map(|val| val.parse::<i64>().unwrap());
            Robot {
                position: Vec2 { x: p_x, y: p_y },
                velocity: Vec2 { x: v_x, y: v_y },
            }
        })
        .collect()
}

// fn display_positions(positions: &[Vec2<i64>]) {
//     println!();
//     let mut pos_idx = Some(positions.len() - 1);
//     for y in 0..AREA_HEIGHT {
//         println!();
//         for x in 0..AREA_WIDTH {
//             let mut occ = 0;
//             while let Some(pos) = pos_idx.map(|i| positions[i]) {
//                 if pos.x == x && pos.y == y {
//                     pos_idx = pos_idx.and_then(|i| i.checked_sub(1));
//                     occ += 1;
//                 } else {
//                     break;
//                 }
//             }
//             print!(
//                 "{}",
//                 if occ == 0 {
//                     ".".to_string()
//                 } else {
//                     occ.to_string()
//                 }
//             );
//         }
//     }
//     println!();
// }
