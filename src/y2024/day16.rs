use crate::{get_text_file, math::Vec2, utils::FxDiGraphMap, SolutionResult};
use ndarray::prelude::*;
use petgraph::{
    algo::{astar, dijkstra},
    prelude::*,
};
use rustc_hash::FxHashMap;
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
const NO_TURN_COST: u64 = 0;
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
    // fn from_vec2(unit_vec: Vec2<isize>) -> Option<Self> {
    //     Some(match unit_vec {
    //         Vec2 { x: 0, y } if y < 0 => Self::Up,
    //         Vec2 { x: 0, y } if y > 0 => Self::Down,
    //         Vec2 { x, y: 0 } if x > 0 => Self::Right,
    //         Vec2 { x, y: 0 } if x < 0 => Self::Left,
    //         _ => None?,
    //     })
    // }

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
            Self::Left => Self::Up,
        }
    }

    fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
}

type Intersection = (Vec2<usize>, Direction);

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (cells, start, end) = read_input(file);

    let maze: FxDiGraphMap<Intersection, u64> = create_maze(&cells, start, end, INITIAL_DIRECTION);
    let start_node = maze.nodes().find(|(idx, _)| *idx == start).unwrap();
    let score = astar(
        &maze,
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
    let file = get_text_file(INPUT_URL)?;
    let (cells, start, end) = read_input(file);
    let maze: FxDiGraphMap<Intersection, u64> = create_maze(&cells, start, end, INITIAL_DIRECTION);

    let start_node = maze.nodes().find(|(idx, _)| *idx == start).unwrap();
    let node_scores = dijkstra(&maze, start_node, None, |e| *e.weight());
    let end_node = node_scores
        .iter()
        .filter(|((idx, _), _)| *idx == end)
        .min_by_key(|(_, score)| *score)
        .unwrap();

    let mut path: FxHashMap<Vec2<usize>, FxHashMap<Vec2<usize>, u64>> = FxHashMap::default();
    let mut check_path = vec![(*end_node.0, *end_node.1)];
    while let Some((node, score)) = check_path.pop() {
        let (index, direction) = node;
        check_path.extend(maze.edges_directed(node, Incoming).filter_map(
            |(neighbor, n2, weight)| {
                assert!(neighbor != node);
                assert!(n2 == node);
                let neighbor_score = node_scores[&neighbor];
                if weight + neighbor_score == score {
                    let (neighbor_index, neighbor_direction) = neighbor;
                    let tiles = if neighbor_direction != direction {
                        weight - TURN_COST
                    } else {
                        *weight
                    };
                    path.entry(neighbor_index)
                        .or_insert(FxHashMap::default())
                        .insert(index, tiles);
                    Some((neighbor, neighbor_score))
                } else {
                    None
                }
            },
        ));
    }
    let tiles = path
        .values()
        .map(|sources| sources.values().sum::<u64>() as i64 - sources.len() as i64 + 1)
        .sum::<i64>()
        + 1;

    Ok(tiles)
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

fn create_maze(
    cells: &Array2<Cell>,
    start: Vec2<usize>,
    end: Vec2<usize>,
    initial_direction: Direction,
) -> FxDiGraphMap<Intersection, u64> {
    let mut maze: FxDiGraphMap<Intersection, u64> = GraphMap::new();
    let mut visited: FxHashMap<Vec2<usize>, Vec<(Direction, u64)>> = FxHashMap::default();

    let get_empty_cell = |index: Vec2<usize>, offset: Vec2<isize>| {
        index
            .signed_add(offset)
            .filter(|i| matches!(cells.get(*i), Some(Cell::Empty)))
    };

    let mut nodes = vec![(start, initial_direction)];
    while let Some(node) = nodes.pop() {
        let (index, direction) = node;

        if let Some(paths) = visited.get(&index) {
            for (next_direction, weight) in paths {
                let next_index = index
                    .signed_add(next_direction.to_unit_vec() * *weight as isize)
                    .unwrap();
                let weight: u64 = weight
                    + if *next_direction == direction {
                        NO_TURN_COST
                    } else if *next_direction == direction.rotate_90()
                        || *next_direction == direction.rotate_90().flip()
                    {
                        TURN_COST
                    } else {
                        continue;
                    };
                let next_node = (next_index, *next_direction);
                if !maze.contains_node(next_node) {
                    nodes.push(next_node);
                }
                maze.add_edge(node, next_node, weight);
            }
        } else {
            let mut edge_weights = Vec::new();

            for (next_direction, turn_weight) in [
                (direction, Some(NO_TURN_COST)),
                (direction.rotate_90(), Some(TURN_COST)),
                (direction.rotate_90().flip(), Some(TURN_COST)),
                (direction.flip(), None),
            ] {
                let step = next_direction.to_unit_vec();
                let mut next_index_opt = get_empty_cell(index, step);
                let mut next_weight = 0;
                while let Some(next_index) = next_index_opt {
                    next_weight += STEP_COST;
                    let side_1 = get_empty_cell(
                        next_index,
                        Vec2 {
                            x: -step.y,
                            y: step.x,
                        },
                    );
                    let side_2 = get_empty_cell(
                        next_index,
                        Vec2 {
                            x: step.y,
                            y: -step.x,
                        },
                    );
                    if side_1.is_some() || side_2.is_some() || next_index == end {
                        edge_weights.push((next_direction, next_weight));
                        if let Some(turn_weight) = turn_weight {
                            let next_node = (next_index, next_direction);
                            if !maze.contains_node(next_node) {
                                nodes.push(next_node);
                            }
                            maze.add_edge(node, next_node, next_weight + turn_weight);
                        }
                        break;
                    }
                    next_index_opt = get_empty_cell(next_index, step);
                }
            }
            visited.insert(index, edge_weights);
        }
    }
    maze
}

// fn display_maze(maze: &FxDiGraphMap<Intersection, u64>) {
//     for node in maze.nodes() {
//         let (idx, direction) = node;
//         println!("<< (x: {}, y: {}), {:?} >>", idx.x, idx.y, direction);
//         print!("Outgoing: ");
//         for (n1, neighbor, weight) in maze.edges_directed(node, Outgoing) {
//             assert!(maze.is_adjacent(&(), n1, neighbor));
//             assert!(n1 == node);
//             // let neighbor = n2;
//             // let neighbor = if n1 == node { n2 } else { n1 };
//             print!(
//                 " ((x: {}, y: {}), {:?}, {})",
//                 neighbor.0.x, neighbor.0.y, neighbor.1, weight
//             );
//         }
//         print!("\nIncoming: ");
//         for (neighbor, n2, weight) in maze.edges_directed(node, Incoming) {
//             assert!(n2 == node);
//             // let neighbor = if n1 == node { n2 } else { n1 };
//             print!(
//                 " ((x: {}, y: {}), {:?}, {})",
//                 neighbor.0.x, neighbor.0.y, neighbor.1, weight
//             );
//         }
//         assert!(maze
//             .edges_directed(node, Outgoing)
//             .all(|i| !maze.edges_directed(node, Incoming).any(|j| i == j)));
//         println!();
//     }
// }

// fn display_cells(cells: &Array2<Cell>, start: Vec2<usize>, end: Vec2<usize>) {
//     println!();
//     for (y, row) in cells.axis_iter(Axis(0)).enumerate() {
//         for (x, cell) in row.iter().enumerate() {
//             let idx = Vec2 { x, y };
//             print!(
//                 "{}",
//                 if start == idx {
//                     START
//                 } else if end == idx {
//                     END
//                 } else {
//                     match cell {
//                         Cell::Empty => EMPTY,
//                         Cell::Wall => WALL,
//                     }
//                 }
//                 .to_string()
//             )
//         }
//         println!();
//     }
// }
