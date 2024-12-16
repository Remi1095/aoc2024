use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use ndarray::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/4/input";

const SOURCE_CHAR: char = 'X';
const OTHER_CHARS: &str = "MAS";

const FIRST_CHAR: char = 'M';
const MIDDLE_CHAR: char = 'A';
const LAST_CHAR: char = 'S';

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let matrix = read_input(file);
    let mut positions = vec![(1, 1), (2, 2), (3, 3)];
    let rotations = (0..8)
        .map(|_| {
            positions = positions.iter().map(|arg0| rotate_45(*arg0)).collect();
            positions.clone()
        })
        .collect_vec();

    let word_found = |idx, rotation: &Vec<(i32, i32)>| {
        OTHER_CHARS
            .chars()
            .zip(rotation.iter())
            .all(|(ch_desired, pos)| {
                if let Some(shift_idx) = add_indices(idx, *pos) {
                    if let Some(ch_actual) = matrix.get(shift_idx) {
                        return *ch_actual == ch_desired;
                    }
                }
                false
            })
    };

    let occurences = matrix
        .indexed_iter()
        .filter(|(_, ch)| **ch == SOURCE_CHAR)
        .map(|(idx, _)| rotations.iter().filter(|r| word_found(idx, *r)).count() as i64)
        .sum();

    Ok(occurences)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let matrix = read_input(file);
    let corner_pairs = [((1, 1), (-1, -1)), ((-1, 1), (1, -1))];

    let word_found = |idx, (pos_1, pos_2): (_, _)| {
        if let (Some(idx_1), Some(idx_2)) = (add_indices(idx, pos_1), add_indices(idx, pos_2)) {
            if let (Some(ch_1), Some(ch_2)) = (matrix.get(idx_1), matrix.get(idx_2)) {
                return matches!(
                    (*ch_1, *ch_2),
                    (FIRST_CHAR, LAST_CHAR) | (LAST_CHAR, FIRST_CHAR)
                );
            }
        }
        false
    };

    let occurences = matrix
        .indexed_iter()
        .filter(|(idx, ch)| {
            **ch == MIDDLE_CHAR
                && corner_pairs
                    .iter()
                    .all(|pos_pair| word_found(*idx, *pos_pair))
        })
        .count() as i64;

    Ok(occurences)
}

fn read_input(file: File) -> Array2<char> {
    let mut data = Vec::new();
    let cols = BufReader::new(file)
        .lines()
        .map(|line| {
            data.extend(line.unwrap().chars());
        })
        .count();
    let rows = data.len() / cols;
    Array2::from_shape_vec((rows, cols), data).unwrap()
}

fn rotate_45((x, y): (i32, i32)) -> (i32, i32) {
    let scale = i32::max(x.abs(), y.abs());
    ((x - y).signum() * scale, (x + y).signum() * scale)
}

// fn rotate_90((x, y): (i32, i32)) -> (i32, i32) {
//     (-y, x)
// }

fn add_indices(idx: (usize, usize), pos: (i32, i32)) -> Option<(usize, usize)> {
    Some((
        (idx.0 as i32 + pos.0).try_into().ok()?,
        (idx.1 as i32 + pos.1).try_into().ok()?,
    ))
}
