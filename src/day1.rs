// use indexset::BTreeMap;
use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/1/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let mut ids_1: Vec<i32> = Vec::new();
    let mut ids_2: Vec<i32> = Vec::new();

    for (id_1, id_2) in iter_input(file) {
        ids_1.push(id_1);
        ids_2.push(id_2);
    }
    ids_1.sort_unstable();
    ids_2.sort_unstable();

    let distance = ids_1
        .iter()
        .zip(ids_2.iter())
        .map(|(id_1, id_2)| (id_1 - id_2).abs())
        .sum();

    Ok(distance)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let mut id_map_1: FxHashMap<i32, usize> = FxHashMap::default();
    let mut id_map_2: FxHashMap<i32, usize> = FxHashMap::default();

    for (id_1, id_2) in iter_input(file) {
        insert_id_fxhashmap(&mut id_map_1, id_1);
        insert_id_fxhashmap(&mut id_map_2, id_2);
    }

    let similarity = id_map_1
        .iter()
        .filter_map(|(id, occ_1)| {
            if let Some(occ_2) = id_map_2.get(id) {
                Some(*id * *occ_1 as i32 * *occ_2 as i32)
            } else {
                None
            }
        })
        .sum();

    Ok(similarity)
}

fn iter_input(file: File) -> impl Iterator<Item = (i32, i32)> {
    BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        line.split_ascii_whitespace()
            .map(|ch| ch.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

fn insert_id_fxhashmap(id_set: &mut FxHashMap<i32, usize>, id: i32) {
    id_set.entry(id).and_modify(|curr| *curr += 1).or_insert(1);
}
