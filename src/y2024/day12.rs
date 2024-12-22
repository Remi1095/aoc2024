use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use ndarray::Array2;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{get_text_file, math::Vec2, SolutionResult};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/12/input";

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let plots = read_input(file);
    let mut plot_ids = Array2::from_shape_simple_fn(plots.raw_dim(), || None);

    #[derive(Debug)]
    struct Region {
        area: u32,
        perimeter: u32,
    }

    let mut regions = Vec::new();
    for (id, (root_index, plot)) in plots.indexed_iter().enumerate() {
        if plot_ids[root_index].is_some() {
            continue;
        }
        let mut region = Region {
            area: 0,
            perimeter: 0,
        };

        let mut plot_stack = vec![root_index];
        plot_ids[root_index] = Some(id);
        while let Some(index) = plot_stack.pop() {
            let (row, col) = index;

            let mut neighbors = 0;
            for other_index in [
                row.checked_sub(1).zip(Some(col)),
                Some(row).zip(col.checked_sub(1)),
                Some(row).zip(Some(col + 1)),
                Some(row + 1).zip(Some(col)),
            ]
            .into_iter()
            .filter_map(|i| i)
            {
                if let Some(other_plot) = plots.get(other_index) {
                    if plot == other_plot {
                        if plot_ids[other_index].is_none() {
                            plot_stack.push(other_index);
                            plot_ids[other_index] = Some(id);
                        }
                        neighbors += 1;
                    }
                }
            }

            region.area += 1;
            region.perimeter += 4 - neighbors as u32;
        }
        regions.push(region);
    }

    let cost = regions
        .into_iter()
        .map(|region| region.area * region.perimeter)
        .sum::<u32>() as i64;

    Ok(cost)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let plots = read_input(file);
    let mut plot_ids = Array2::from_shape_simple_fn(plots.raw_dim(), || None);
    let mut visited_plot = Array2::from_shape_simple_fn(plots.raw_dim(), || false);

    #[derive(Debug)]
    struct Region {
        area: u32,
        sides: u32,
    }

    let mut regions = Vec::new();

    for (id, (root_index, plot)) in plots.indexed_iter().enumerate() {
        if visited_plot[root_index] {
            continue;
        }
        let mut region = Region { area: 0, sides: 0 };
        let mut plot_stack = Vec::new();
        plot_stack.push(Vec2::from_index_tuple(root_index));
        visited_plot[root_index] = true;
        while let Some(index) = plot_stack.pop() {
            plot_ids[index] = Some(id);

            let is_neighbor = |offset: Vec2<isize>| {
                index
                    .signed_add(offset)
                    .and_then(|other_index| {
                        plot_ids
                            .get(other_index)
                            .map(|other_id| *other_id == Some(id))
                    })
                    .unwrap_or(false)
            };

            let sides: i32 = [
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 0, y: -1 },
                Vec2 { x: -1, y: 0 },
            ]
            .into_iter()
            .map(|top_mid| {
                if let Some(other_index) = index.signed_add(top_mid) {
                    if plots
                        .get(other_index)
                        .map(|other_plot| plot == other_plot && !visited_plot[other_index])
                        .unwrap_or(false)
                    {
                        plot_stack.push(other_index);
                        visited_plot[other_index] = true;
                    }
                }

                let mid_left = Vec2 {
                    x: -top_mid.y,
                    y: top_mid.x,
                };
                let mid_right = -mid_left;
                let top_left = mid_left + top_mid;
                let top_right = mid_right + top_mid;

                let top_mid = is_neighbor(top_mid);
                let mid_left = is_neighbor(mid_left);
                let mid_right = is_neighbor(mid_right);
                let top_left = is_neighbor(top_left) && (top_mid != mid_left);
                let top_right = is_neighbor(top_right) && (top_mid != mid_right);

                match (top_mid, mid_left, mid_right, top_left, top_right) {
                    (false, true, true, false, false) => -1,
                    (false, true, _, false, _) | (false, _, true, _, false) => 0,
                    (true, false, false, true, true) => 1,
                    (true, false, _, true, _) | (true, _, false, _, true) => 0,
                    (true, _, _, _, _) => -1,
                    _ => 1,
                }
            })
            .sum();

            region.area += 1;
            region.sides += sides as u32;
        }
        regions.push(region);
    }

    let cost = regions
        .into_iter()
        .map(|region| region.area * region.sides)
        .sum::<u32>() as i64;

    Ok(cost)
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
