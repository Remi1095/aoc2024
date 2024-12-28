use crate::{get_text_file, math::Vec2, utils::FxUnGraphMap, SolutionResult};
use itertools::Itertools;
use ndarray::prelude::*;
use petgraph::{algo::astar, prelude::*};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/16/input";

const EMPTY: char = '.';
const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

const STEP_COST: u64 = 1;
const TURN_COST: u64 = 1000;
const INITIAL_DIRECTION: Direction = Direction::Right;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_unit_vec(unit_vec: Vec2<isize>) -> Option<Self> {
        Some(match unit_vec {
            Vec2 { x: 0, y: -1 } => Self::Up,
            Vec2 { x: 1, y: 0 } => Self::Right,
            Vec2 { x: 0, y: 1 } => Self::Down,
            Vec2 { x: -1, y: 0 } => Self::Left,
            _ => None?,
        })
    }

    fn to_unit_vec(&self) -> Vec2<isize> {
        match self {
            Self::Up => Vec2 { x: 0, y: -1 },
            Self::Right => Vec2 { x: 1, y: 0 },
            Self::Down => Vec2 { x: 0, y: 1 },
            Self::Left => Vec2 { x: -1, y: 0 },
        }
    }

    fn rotate_90(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left =>Self::Up,
        }
    }

    fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left =>Self::Right,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (cells, start, end) = read_input(file);

    let mut no_turn_maze: FxUnGraphMap<Vec2<usize>, u64> = GraphMap::new();

    let get_empty_cell = |index: Vec2<usize>, offset: Vec2<isize>| {
        index
            .signed_add(offset)
            .filter(|i| matches!(cells.get(*i), Some(Cell::Empty)))
    };

    let mut indicies = vec![(start, INITIAL_DIRECTION.to_unit_vec())];
    while let Some((index, step)) = indicies.pop() {
        for next_step in [
            step,
            step.rotate_90(),
            -step.rotate_90(),
        ] {
            let mut next_index_opt = get_empty_cell(index, next_step);
            let mut next_weight = 0;
            while let Some(next_index) = next_index_opt {
                next_weight += STEP_COST;
                let side_1 = get_empty_cell(
                    next_index,
                    Vec2 {
                        x: -next_step.y,
                        y: next_step.x,
                    },
                );
                let side_2 = get_empty_cell(
                    next_index,
                    Vec2 {
                        x: next_step.y,
                        y: -next_step.x,
                    },
                );
                if side_1.is_some() || side_2.is_some() || next_index == end {
                    if !no_turn_maze.contains_node(next_index) {
                        // println!("already exists {:?}", next_index);
                        indicies.push((next_index, next_step));
                    }
                    no_turn_maze.add_edge(
                        index,
                        next_index,
                        next_weight,
                    );
                    break;
                }
                next_index_opt = get_empty_cell(next_index, next_step);
            }
        }
    }

    let mut turn_maze: FxUnGraphMap<(Vec2<usize>, Direction), u64> = GraphMap::new();
    turn_maze.add_node((start, INITIAL_DIRECTION));
    let new_nodes = vec![(start, INITIAL_DIRECTION)];
    while let Some(node) = new_nodes.pop() {
        let (index, direction) = node;
        for next_direction in [
            direction,
            direction.rotate_90(),
            direction.rotate_90().flip(),
        ] {
            
        }
    }


    // display_maze(&no_turn_maze, &cells, start, end);
    // let start_node = no_turn_maze.nodes().find(|(idx, _)| *idx == start).unwrap();
    // println!(
    //     "start: {:?} neighbors: {:?}",
    //     start_node,
    //     no_turn_maze.neighbors(start_node).collect_vec()
    // );
    // let end_nodes = no_turn_maze
    //     .nodes()
    //     .filter(|(idx, _)| *idx == end)
    //     .collect_vec();
    // println!(
    //     "end: {:?} neighbors: {:?}",
    //     end_nodes,
    //     end_nodes
    //         .iter()
    //         .map(|n| no_turn_maze.edges(*n).collect_vec()),
    // );
    let score = astar(
        &no_turn_maze,
        start_node,
        |(idx, _)| idx == end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap()
    .0 as i64;

    Ok(score)
}

pub fn part_2() -> SolutionResult {
    Ok(0)
}

fn read_input(file: File) -> (Array2<Cell>, Vec2<usize>, Vec2<usize>) {
    let mut cells = Vec::new();
    let mut start = None;
    let mut end = None;

    let rows = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(row, line)| {
            cells.extend(line.unwrap().chars().enumerate().filter_map(|(col, ch)| {
                Some(match ch {
                    EMPTY => Cell::Empty,
                    WALL => Cell::Wall,
                    START => {
                        start = Some(Vec2 { x: col, y: row });
                        Cell::Empty
                    }
                    END => {
                        end = Some(Vec2 { x: col, y: row });
                        Cell::Empty
                    }
                    _ => None?,
                })
            }))
        })
        .count();

    let cols = cells.len() / rows;
    (
        Array2::from_shape_vec((rows, cols), cells).unwrap(),
        start.unwrap(),
        end.unwrap(),
    )
}

fn display_maze(
    maze: &FxUnGraphMap<(Vec2<usize>, Direction), u64>,
    cells: &Array2<Cell>,
    start: Vec2<usize>,
    end: Vec2<usize>,
) {
    for node in maze.nodes() {
        let (idx, direction) = node;
        print!("(x: {}, y: {}), {:?}:", idx.x, idx.y, direction);
        for (n1, n2, weight) in maze.edges(node) {
            let neighbor = if n1 == node { n2 } else { n1 };
            print!(
                " ((x: {}, y: {}), {:?}, {})",
                neighbor.0.x, neighbor.0.y, neighbor.1, weight
            );
        }
        println!();
    }
    println!();
    for (y, row) in cells.axis_iter(Axis(0)).enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let idx = Vec2 { x, y };
            print!(
                "{}",
                if start == idx {
                    START
                } else if end == idx {
                    END
                } else {
                    match cell {
                        Cell::Empty => EMPTY,
                        Cell::Wall => WALL,
                    }
                }
                .to_string()
            )
        }
        println!();
    }
}
