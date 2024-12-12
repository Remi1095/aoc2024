use crate::{get_text_file, AnyError, SolutionResult};
use itertools::Itertools;
use ndarray::Array2;
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
    let matrix = read_input_matrix(file)?;
    let mut positions = vec![(1, 1), (2, 2), (3, 3)];
    let rotations = (0..8)
        .map(|_| {
            positions = positions.iter().map(|arg0| rotate_45(*arg0)).collect();
            positions.clone()
        })
        .collect_vec();
    let mut occurences = 0;
    for (idx, _) in matrix.indexed_iter().filter(|(_, ch)| **ch == SOURCE_CHAR) {
        for rotation in &rotations {
            if OTHER_CHARS.chars().zip(rotation).all(|(ch_desired, pos)| {
                if let Some(shift_idx) = add_indices(idx, *pos) {
                    if let Some(ch_actual) = matrix.get(shift_idx) {
                        return *ch_actual == ch_desired;
                    }
                }
                false
            }) {
                occurences += 1;
            }
        }
    }

    Ok(occurences)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let matrix = read_input_matrix(file)?;
    let corner_pairs = [((1, 1), (-1, -1)), ((-1, 1), (1, -1))];
    let mut occurences = 0;
    for (idx, _) in matrix.indexed_iter().filter(|(_, ch)| **ch == MIDDLE_CHAR) {
        if corner_pairs.iter().all(|(pos_1, pos_2)| {
            if let (Some(idx_1), Some(idx_2)) = (add_indices(idx, *pos_1), add_indices(idx, *pos_2))
            {
                if let (Some(ch_1), Some(ch_2)) = (matrix.get(idx_1), matrix.get(idx_2)) {
                    return matches!(
                        (*ch_1, *ch_2),
                        (FIRST_CHAR, LAST_CHAR) | (LAST_CHAR, FIRST_CHAR)
                    );
                }
            }
            false
        }) {
            occurences += 1;
        }
    }

    Ok(occurences)
}

fn read_input_matrix(file: File) -> Result<Array2<char>, AnyError> {
    let mut data = Vec::new();
    let mut cols = 0;
    for line in BufReader::new(file).lines() {
        data.extend(line?.chars());
        cols += 1;
    }
    let rows = data.len() / cols;
    Ok(Array2::from_shape_vec((rows, cols), data).unwrap())
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
