use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::{get_text_file, SolutionResult, INPUT_DIR};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/2/input";

const MIN_LEVEL_DIFF: i32 = 1;
const MAX_LEVEL_DIFF: i32 = 3;

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL, INPUT_DIR)?;
    let mut safe_reports = 0;

    for levels in iter_input(file) {
        if levels.len() <= 1 {
            safe_reports += 1;
            continue;
        }
        let sign = match levels[0].cmp(&levels[1]) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => continue,
        };
        if levels
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(|(l1, l2)| sign * (l2 - l1))
            .all(|diff| (MIN_LEVEL_DIFF..=MAX_LEVEL_DIFF).contains(&diff))
        {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL, INPUT_DIR)?;
    let mut safe_reports = 0;

    let is_good = |diff| (MIN_LEVEL_DIFF..=MAX_LEVEL_DIFF).contains(&diff);

    for levels in iter_input(file) {
        // println!("levels {:?}", levels);
        if levels.len() < 2 {
            safe_reports += 1;
            continue;
        }
        let mut levels_diff: Vec<_> = levels
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(|(l1, l2)| l2 - l1)
            .collect();

        let mut positives = 0;
        let mut negatives = 0;
        for diff in levels_diff.iter() {
            match diff.cmp(&0) {
                Ordering::Less => negatives += 1,
                Ordering::Greater => positives += 1,
                Ordering::Equal => {}
            };
        }
        let sign = if positives >= negatives { 1 } else { -1 };
        levels_diff.iter_mut().for_each(|diff| *diff = sign * *diff);

        let mut idx = 0;
        let mut tolerate = true;
        let mut safe = true;
        while idx < levels_diff.len() {
            let diff = levels_diff[idx];
            if !is_good(diff) {
                if !tolerate {
                    safe = false;
                    break;
                }

                let tolerate_left = idx == 0 || is_good(diff + levels_diff[idx - 1]);
                let tolerate_right =
                    idx + 1 == levels_diff.len() || is_good(diff + levels_diff[idx + 1]);

                if tolerate_right {
                    idx += 2;
                } else if tolerate_left {
                    idx += 1;
                } else {
                    safe = false;
                    break;
                }
                tolerate = false;
            } else {
                idx += 1;
            }
        }
        if safe {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}

fn iter_input(file: File) -> impl Iterator<Item = Vec<i32>> {
    BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        line.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}
