use crate::{get_text_file, math::Vec2, SolutionResult};
use itertools::Itertools;
use ndarray::Array2;
use petgraph::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/10/input";

const TRAILHEAD: i32 = 0;
const TRAILTAIL: i32 = 9;

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let (topographic_map, trailheads) = read_input(file);
    let trail_seqence: FxHashMap<_, _> = (TRAILHEAD..=TRAILTAIL).tuple_windows().collect();
    // println!("trail_seqence {:?}", trail_seqence);

    let score = trailheads
        .into_iter()
        .map(|trailhead| {
            // println!();
            // println!("trailhead {:?}", trailhead);
            let mut trail_item = TRAILHEAD;
            let mut positions = FxHashSet::default();
            positions.insert(trailhead);
            // println!();
            // println!("trail_item {:?}", trail_item);
            // println!("positions {:?}", positions);
            while !positions.is_empty() {
                positions = FxHashSet::from_iter(positions.into_iter().flat_map(|position| {
                    iter_neighbors(position).filter(|neighbor| {
                        if let Some(neighbor_item) = topographic_map.get(*neighbor) {
                            trail_seqence[&trail_item] == *neighbor_item
                        } else {
                            false
                        }
                    })
                }));

                trail_item = trail_seqence[&trail_item];
                // println!();
                // println!("trail_item {:?}", trail_item);
                // println!("positions {:?}", positions);
                if trail_item == TRAILTAIL {
                    // println!("score {}", positions.len());
                    return positions.len();
                }
            }
            // println!("score 0");
            0
        })
        .sum::<usize>() as i64;

    Ok(score)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    let (topographic_map, trailheads) = read_input(file);
    let trail_seqence: FxHashMap<_, _> = (TRAILHEAD..=TRAILTAIL).tuple_windows().collect();

    let rating = trailheads
        .into_iter()
        .map(|trailhead| {
            let mut trail_item = TRAILHEAD;

            let mut positions = FxHashSet::default();
            positions.insert(trailhead);

            let mut position_paths = FxHashMap::default();
            position_paths.insert(trailhead, 1);

            // println!();
            // println!("trail_item {:?}", trail_item);
            // println!("positions {:?}", positions);
            // println!("position_paths {:?}", position_paths);
            while !positions.is_empty() {
                positions = FxHashSet::from_iter(positions.into_iter().flat_map(|node| {
                    iter_neighbors(node)
                        .filter(|neighbor| {
                            if let Some(neighbor_item) = topographic_map.get(*neighbor) {
                                if trail_seqence[&trail_item] == *neighbor_item {
                                    let node_paths = position_paths[&node];
                                    position_paths
                                        .entry(*neighbor)
                                        .and_modify(|p| *p += node_paths)
                                        .or_insert(node_paths);
                                    return true;
                                }
                            }
                            false
                        })
                        .collect_vec()
                }));

                trail_item = trail_seqence[&trail_item];
                // println!();
                // println!("trail_item {:?}", trail_item);
                // println!("positions {:?}", positions);
                // println!("position_paths {:?}", position_paths);
                if trail_item == TRAILTAIL {
                    // println!("rating {:?}", positions.iter().map(|node| position_paths[node]).sum::<usize>());
                    return positions.iter().map(|node| position_paths[node]).sum();
                }
            }
            // println!("rating 0");
            0
        })
        .sum::<usize>() as i64;

    Ok(rating)
}

fn read_input(file: File) -> (Array2<i32>, Vec<Vec2<usize>>) {
    let mut positions = Vec::new();
    let mut trailtails = Vec::new();

    let rows = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(row, line)| {
            positions.extend(line.unwrap().chars().enumerate().filter_map(|(col, ch)| {
                if let Some(val) = ch.to_digit(10) {
                    if val == TRAILHEAD as u32 {
                        trailtails.push(Vec2 { x: col, y: row });
                    }
                    Some(val as i32)
                } else {
                    None
                }
            }));
        })
        .count();

    let cols = positions.len() / rows;
    (
        Array2::from_shape_vec((rows, cols), positions).unwrap(),
        trailtails,
    )
}

fn iter_neighbors(idx: Vec2<usize>) -> impl Iterator<Item = Vec2<usize>> {
    [
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 0, y: -1 },
    ]
    .into_iter()
    .filter_map(move |offset: Vec2<isize>| {
        (idx.convert::<isize>().unwrap() + offset).convert::<usize>()
    })
}
