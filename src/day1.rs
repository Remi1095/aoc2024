use indexset::BTreeMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use rustc_hash::FxHashMap;

use crate::{get_text_file, SolutionResult, INPUT_DIR};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/1/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL, INPUT_DIR)?;
    let mut id_set_1: BTreeMap<i32, usize> = BTreeMap::new();
    let mut id_set_2: BTreeMap<i32, usize> = BTreeMap::new();

    for (id_1, id_2) in iter_input(file) {
        insert_id_btreemap(&mut id_set_1, id_1);
        insert_id_btreemap(&mut id_set_2, id_2);
    }

    let mut id_iter_1 = id_set_1.into_iter();
    let mut id_iter_2 = id_set_2.into_iter();

    let mut distance = 0;
    let mut id_item_1 = id_iter_1.next();
    let mut id_item_2 = id_iter_2.next();
    while let (Some((id_1, occ_1)), Some((id_2, occ_2))) = (&mut id_item_1, &mut id_item_2) {
        distance += (*id_2 - *id_1).abs();
        if *occ_1 == 1 {
            id_item_1 = id_iter_1.next();
        } else {
            *occ_1 -= 1;
        }
        if *occ_2 == 1 {
            id_item_2 = id_iter_2.next();
        } else {
            *occ_2 -= 1;
        }
    }

    Ok(distance)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL, INPUT_DIR)?;

    let mut id_set_1: FxHashMap<i32, usize> = FxHashMap::default();
    let mut id_set_2: FxHashMap<i32, usize> = FxHashMap::default();

    for (id_1, id_2) in iter_input(file) {
        insert_id_fxhashmap(&mut id_set_1, id_1);
        insert_id_fxhashmap(&mut id_set_2, id_2);
    }

    // println!("id_set_1 {:?}", id_set_1);
    // println!("id_set_2 {:?}", id_set_2);

    let similarity = id_set_1.iter().fold(0, |acc, (id, occ_1)| {
        if let Some(occ_2) = id_set_2.get(id) {
            acc + *id * *occ_1 as i32 * *occ_2 as i32
        } else {
            acc
        }
    });

    Ok(similarity)
}

fn iter_input(file: File) -> impl Iterator<Item = (i32, i32)> {
    BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut ids = line.split_whitespace();
        (
            ids.next().unwrap().parse::<i32>().unwrap(),
            ids.next().unwrap().parse::<i32>().unwrap(),
        )
    })
}

fn insert_id_btreemap(id_set: &mut BTreeMap<i32, usize>, id: i32) {
    id_set.entry(id).and_modify(|curr| *curr += 1).or_insert(1);
}

fn insert_id_fxhashmap(id_set: &mut FxHashMap<i32, usize>, id: i32) {
    id_set.entry(id).and_modify(|curr| *curr += 1).or_insert(1);
}
