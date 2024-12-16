use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use petgraph::{
    algo::{has_path_connecting, toposort},
    prelude::*,
    visit::{IntoEdges, NodeFiltered},
};
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type OrderGraph = GraphMap<i32, (), Directed, FxBuildHasher>;

const INPUT_URL: &str = "https://adventofcode.com/2024/day/5/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (graph, sequences) = read_input(file);

    let result = sequences
        .into_iter()
        .filter_map(|values| {
            let value_set: FxHashSet<i32> = values.clone().into_iter().collect();
            let filtered = NodeFiltered::from_fn(&graph, |n| value_set.contains(&n));

            let sorted = toposort(&filtered, None).unwrap();

            if *values == sorted {
                values
                    .get(values.len() / 2 + values.len() % 2 - 1)
                    .map(i32::clone)
            } else {
                None
            }
        })
        .sum();

    Ok(result)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (graph, sequences) = read_input(file);
    // let order: FxHashMap<i32, usize> = toposort(&graph, None)
    //     .unwrap()
    //     .into_iter()
    //     .enumerate()
    //     .map(|(idx, val)| (val, idx))
    //     .collect();

    let result = sequences
        .into_iter()
        .map(|values| {
            let value_set: FxHashSet<i32> = values.clone().into_iter().collect();
            let filtered = NodeFiltered::from_fn(&graph, |n| value_set.contains(&n));

            let sorted = toposort(&filtered, None).unwrap();

            // let values_clone = values.clone();
            // let compare = |a: &_, b: &_| {
            //     if has_path_connecting(&filtered, *a, *b, None) {
            //         Ordering::Less
            //     } else if has_path_connecting(&filtered, *a, *b, None) {
            //         Ordering::Greater
            //     } else {
            //         Ordering::Equal
            //     }
            // };

            let middle_idx = values.len() / 2 + values.len() % 2 - 1;
            if sorted != *values {
                println!("");
                println!("UNSORTED");
                println!(
                    "filtered {:?}",
                    values
                        .iter()
                        .map(|a| (a, filtered.edges(*a).map(|(_, b, _)| b).collect_vec()))
                        .collect_vec()
                );
                println!("values {:?}", values);
                println!("sorted {:?}", sorted);
                println!("median {}", sorted[middle_idx]);
                assert!(sorted
                    .iter()
                    .tuple_combinations()
                    .all(|(a, b)| graph.contains_edge(*a, *b)));
            }
            sorted[middle_idx]
        })
        .sum();

    Ok(result)
}

pub fn read_input(file: File) -> (OrderGraph, Vec<Vec<i32>>) {
    let mut first_section = true;
    let mut graph = OrderGraph::new();
    let mut sequences: Vec<Vec<i32>> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            first_section = false;
        } else if first_section {
            let (v, u) = line
                .split('|')
                .map(|ch| ch.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            graph.add_edge(v, u, ());
        } else {
            sequences.push(
                line.split(',')
                    .map(|ch| ch.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }

    (graph, sequences)
}
