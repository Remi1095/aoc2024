use crate::{get_text_file, SolutionResult};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::BTreeMap, error::Error, fs::File, io::Read, iter, vec};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/11/input";

const NUM_BLINKS_1: usize = 25;
const NUM_BLINKS_2: usize = 75;
const SCALE_FACTOR: i64 = 2024;

#[derive(Debug, Clone)]
struct ComposedNodes(FxHashMap<(i64, usize), usize>);

impl ComposedNodes {
    fn insert(&mut self, node: i64, offset: usize, quantity: usize) {
        self.0
            .entry((node, offset))
            .and_modify(|q| *q += quantity)
            .or_insert(quantity);
    }

    fn add_offsets(&mut self, amount: usize) {
        self.0 = self
            .0
            .drain()
            .map(|((node, offset), quantity)| ((node, offset + amount), quantity))
            .collect();
    }

    fn multiply_quantities(&mut self, factor: usize) {
        for (_, quantity_) in &mut self.0 {
            *quantity_ *= factor;
        }
    }

    fn merge(&mut self, others: Vec<Self>) {
        for ((node, offset), quantity) in others.into_iter().flat_map(|c| c.0) {
            self.insert(node, offset, quantity);
        }
    }
}

#[derive(Debug)]
struct NodeTree {
    blink_counts: Vec<usize>,
    leaves: ComposedNodes,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let mut stones = read_input(file)?;
    // println!("stones {:?}", stones);
    for _ in 0..NUM_BLINKS_1 {
        let stones_len = stones.len();
        for idx in 0..stones_len {
            let n = &mut stones[idx];
            if *n == 0 {
                *n = 1;
            } else {
                let digits = get_digits(*n);
                // println!("digits {:?}", digits);
                if digits.len() % 2 == 0 {
                    let mid = digits.len() / 2;
                    *n = digit_to_value(&digits[mid..]);
                    stones.push(digit_to_value(&digits[..mid]));
                } else {
                    *n *= SCALE_FACTOR;
                }
            }
        }
        // println!("stones {:?}", stones);
    }
    Ok(stones.len() as i64)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let stones = read_input(file)?;

    let nodes = (0..=9).collect_vec();
    let node_trees = compute_node_trees(&nodes);

    let node_depth_table = compute_node_depth_table(&nodes, &node_trees);

    let mut num_stones = 0;
    for root in stones {
        let mut sub_stones = vec![root];
        for num_blinks in (1..=NUM_BLINKS_2).rev() {
            sub_stones = sub_stones
                .into_iter()
                .filter(|node| {
                    if let Some(num) =
                        get_node_num_stones(&node_trees, &node_depth_table, *node, num_blinks)
                    {
                        num_stones += num;
                        false
                    } else {
                        true
                    }
                })
                .flat_map(|node| blink(node))
                .collect();
            if sub_stones.is_empty() {
                break;
            }
        }
        if !sub_stones.is_empty() {
            num_stones += sub_stones.len();
        }
    }

    Ok(num_stones as i64)
}

fn read_input(mut file: File) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text
        .split_ascii_whitespace()
        .map(|ch| ch.parse().unwrap())
        .collect())
}

fn compute_node_trees(nodes: &[i64]) -> FxHashMap<i64, NodeTree> {
    let nodes: FxHashSet<_> = nodes.into_iter().collect();
    nodes
        .iter()
        .map(|root| {
            let mut stones = vec![(**root, false)];
            let mut blink_counts = vec![1];
            let mut leaves = ComposedNodes(Default::default());
            while !stones.is_empty() {
                leaves.add_offsets(1);

                stones = stones
                    .into_iter()
                    .flat_map(|(n, ignore)| blink(n).into_iter().map(move |n| (n, ignore)))
                    .collect();

                for (n, ignore) in &mut stones {
                    if !*ignore && nodes.contains(n) {
                        leaves.insert(*n, 0, 1);
                        *ignore = true;
                    }
                }
                if stones.iter().all(|(_, ignore)| *ignore) {
                    break;
                }
                blink_counts.push(stones.len());
            }
            (
                **root,
                NodeTree {
                    blink_counts,
                    leaves,
                },
            )
        })
        .collect()
}

fn compute_node_depth_table(
    nodes: &[i64],
    node_trees: &FxHashMap<i64, NodeTree>,
) -> BTreeMap<i64, Vec<ComposedNodes>> {
    let mut node_depth_table: BTreeMap<i64, Vec<ComposedNodes>> =
        nodes.iter().map(|b| (*b, Vec::new())).collect();

    for _ in 0..=NUM_BLINKS_2 {
        for (node, composed_nodes_vec) in &mut node_depth_table {
            if let Some(mut composed_nodes) = composed_nodes_vec.last().cloned() {
                composed_nodes.add_offsets(1);

                let mut sub_composed_nodes_vec = Vec::new();
                let mut to_remove = Vec::new();

                for ((sub_node, offset), quantity) in &mut composed_nodes.0 {
                    let sub_node_tree = &node_trees[sub_node];
                    let depth = sub_node_tree.blink_counts.len();
                    if *offset == depth {
                        let mut leaves = sub_node_tree.leaves.clone();
                        leaves.multiply_quantities(*quantity);
                        sub_composed_nodes_vec.push(leaves);
                        to_remove.push((*sub_node, *offset));
                    }
                    assert!(*offset <= depth);
                }
                for key in &to_remove {
                    composed_nodes.0.remove(key);
                }
                composed_nodes.merge(sub_composed_nodes_vec);
                composed_nodes_vec.push(composed_nodes);
            } else {
                composed_nodes_vec.push(ComposedNodes(iter::once(((*node, 0), 1)).collect()));
            }
        }
    }
    node_depth_table
}

fn get_node_num_stones(
    node_trees: &FxHashMap<i64, NodeTree>,
    node_depth_table: &BTreeMap<i64, Vec<ComposedNodes>>,
    node: i64,
    num_blinks: usize,
) -> Option<usize> {
    let composed_nodes = node_depth_table.get(&node)?.get(num_blinks)?;
    Some(
        composed_nodes
            .0
            .iter()
            .map(|((sub_node, offset), quantity)| {
                let sub_node_tree = &node_trees[sub_node];
                quantity * sub_node_tree.blink_counts[*offset]
            })
            .sum(),
    )
}

fn blink(n: i64) -> Vec<i64> {
    if n == 0 {
        vec![1]
    } else {
        let digits = get_digits(n);
        // println!("digits {:?}", digits);
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            vec![
                digit_to_value(&digits[mid..]),
                digit_to_value(&digits[..mid]),
            ]
        } else {
            vec![n * SCALE_FACTOR]
        }
    }
}

fn get_digits(n: i64) -> Vec<i64> {
    let mut num = n.abs();
    if num == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

fn digit_to_value(digits: &[i64]) -> i64 {
    digits
        .iter()
        .enumerate()
        .map(|(idx, d)| d * 10_i64.pow(idx as u32))
        .sum()
}
