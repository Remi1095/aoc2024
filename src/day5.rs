use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use petgraph::{algo::toposort, prelude::*, visit::NodeFiltered};
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
                let middle_idx = values.len() / 2 + values.len() % 2 - 1;
                values.get(middle_idx).map(i32::clone)
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

    let result = sequences
        .into_iter()
        .filter_map(|values| {
            let value_set: FxHashSet<i32> = values.clone().into_iter().collect();
            let filtered = NodeFiltered::from_fn(&graph, |n| value_set.contains(&n));

            let sorted = toposort(&filtered, None).unwrap();

            if *values != sorted {
                let middle_idx = values.len() / 2 + values.len() % 2 - 1;
                sorted.get(middle_idx).map(i32::clone)
            } else {
                None
            }
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
