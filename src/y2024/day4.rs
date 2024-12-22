use crate::{get_text_file, math::Vec2, SolutionResult};
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
    let mut positions = (1..=3).map(|i: isize| Vec2 { x: i, y: i }).collect_vec();
    let rotations = (0..8)
        .map(|_| {
            positions = positions.iter().map(|arg0| rotate_45(*arg0)).collect();
            positions.clone()
        })
        .collect_vec();
    let word_found = |idx: Vec2<usize>, rotation: &Vec<Vec2<isize>>| {
        OTHER_CHARS
            .chars()
            .zip(rotation.iter())
            .all(|(ch_desired, pos)| {
                idx.signed_add(*pos)
                    .and_then(|shift_idx| matrix.get(shift_idx))
                    .map_or(false, |ch_actual| *ch_actual == ch_desired)
            })
    };

    let occurences = matrix
        .indexed_iter()
        .filter(|(_, ch)| **ch == SOURCE_CHAR)
        .map(|(idx, _)| {
            rotations
                .iter()
                .filter(|r| word_found(Vec2::from_index_tuple(idx), *r))
                .count() as i64
        })
        .sum();

    Ok(occurences)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let matrix = read_input(file);
    let mut corner = Vec2 { x: 1, y: 1 };
    let corner_pairs = (0..2)
        .map(|_| {
            corner = rotate_90(corner);
            (corner, -corner)
        })
        .collect_vec();
    let word_found = |idx: Vec2<usize>, (pos_1, pos_2): (_, _)| {
        idx.signed_add(pos_1)
            .zip(idx.signed_add(pos_2))
            .and_then(|(idx_1, idx_2)| matrix.get(idx_1).zip(matrix.get(idx_2)))
            .map_or(false, |(ch_1, ch_2)| {
                matches!(
                    (*ch_1, *ch_2),
                    (FIRST_CHAR, LAST_CHAR) | (LAST_CHAR, FIRST_CHAR)
                )
            })
    };

    let occurences = matrix
        .indexed_iter()
        .filter(|(idx, ch)| {
            **ch == MIDDLE_CHAR
                && corner_pairs
                    .iter()
                    .all(|pos_pair| word_found(Vec2::from_index_tuple(*idx), *pos_pair))
        })
        .count() as i64;

    Ok(occurences)
}

fn read_input(file: File) -> Array2<char> {
    let mut data = Vec::new();
    let rows = BufReader::new(file)
        .lines()
        .map(|line| {
            data.extend(line.unwrap().chars());
        })
        .count();
    let cols = data.len() / rows;
    Array2::from_shape_vec((rows, cols), data).unwrap()
}

fn rotate_45(Vec2 { x, y }: Vec2<isize>) -> Vec2<isize> {
    let scale = isize::max(x.abs(), y.abs());
    Vec2 {
        x: (x - y).signum() * scale,
        y: (x + y).signum() * scale,
    }
}

fn rotate_90(Vec2 { x, y }: Vec2<isize>) -> Vec2<isize> {
    Vec2 { x: -y, y: x }
}
