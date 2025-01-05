use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::HashSet,
    fs::File,
    hash::BuildHasher,
    io::{BufRead, BufReader},
};

use crate::{get_text_file, math::Vec2, SolutionResult};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/8/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (antenna_types, bounds) = read_input(file);
    let mut antinodes = FxHashSet::<Vec2<isize>>::default();
    for antennas in antenna_types {
        for (antenna_1, antenna_2) in antennas
            .into_iter()
            .combinations(2)
            .map(|x| x.into_iter().collect_tuple::<(_, _)>().unwrap())
        {
            insert_antinode_pair(antenna_1, antenna_2, bounds, &mut antinodes);
        }
    }
    Ok(antinodes.len().to_string())
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (antenna_types, bounds) = read_input(file);
    let mut antinodes = FxHashSet::<Vec2<isize>>::default();
    for antennas in antenna_types {
        for (antenna_1, antenna_2) in antennas
            .into_iter()
            .combinations(2)
            .map(|x| x.into_iter().collect_tuple::<(_, _)>().unwrap())
        {
            insert_antinodes(antenna_1, antenna_2, bounds, &mut antinodes);
        }
    }
    Ok(antinodes.len().to_string())
}

fn read_input(file: File) -> (Vec<Vec<Vec2<isize>>>, (Vec2<isize>, Vec2<isize>)) {
    let mut map = FxHashMap::<char, Vec<Vec2<isize>>>::default();

    let mut lines_iter = BufReader::new(file).lines().peekable();

    let cols = lines_iter.peek().unwrap().as_ref().unwrap().chars().count() as isize;

    let rows = lines_iter
        .enumerate()
        .map(|(row, line)| {
            let line = line.unwrap();
            for (col, ch) in line.char_indices().filter(|(_, ch)| *ch != '.') {
                map.entry(ch).or_insert(Vec::new()).push(Vec2 {
                    x: col as isize,
                    y: row as isize,
                });
            }
        })
        .count() as isize;

    (
        map.into_values().collect(),
        (Vec2 { x: 0, y: 0 }, Vec2 { x: cols, y: rows }),
    )
}

fn insert_antinode_pair<S: BuildHasher>(
    antenna_1: Vec2<isize>,
    antenna_2: Vec2<isize>,
    bounds: (Vec2<isize>, Vec2<isize>),
    antinodes: &mut HashSet<Vec2<isize>, S>,
) {
    let diff = antenna_2 - antenna_1;
    let antinode_1 = antenna_1 - diff;
    let antinode_2 = antenna_2 + diff;
    if antinode_1.in_bounds(bounds) {
        antinodes.insert(antinode_1);
    }
    if antinode_2.in_bounds(bounds) {
        antinodes.insert(antinode_2);
    }
}

fn insert_antinodes<S: BuildHasher>(
    antenna_1: Vec2<isize>,
    antenna_2: Vec2<isize>,
    bounds: (Vec2<isize>, Vec2<isize>),
    antinodes: &mut HashSet<Vec2<isize>, S>,
) {
    let diff = antenna_2 - antenna_1;
    let mut antinode_1 = antenna_1;
    while antinode_1.in_bounds(bounds) {
        antinodes.insert(antinode_1);
        antinode_1 = antinode_1 - diff;
    }
    let mut antinode_2 = antenna_2;
    while antinode_2.in_bounds(bounds) {
        antinodes.insert(antinode_2);
        antinode_2 = antinode_2 + diff;
    }
}
