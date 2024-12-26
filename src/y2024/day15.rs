use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{get_text_file, math::Vec2, SolutionResult};
use itertools::{Either, Itertools};
use ndarray::{Array2, Axis};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/15/input";

const EMPTY: char = '.';
const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const UP: char = '^';
const RIGHT: char = '>';
const DOWN: char = 'v';
const LEFT: char = '<';

const GPS_FACTOR: usize = 100;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn new(symbol: char) -> Option<Self> {
        Some(match symbol {
            UP => Self::Up,
            RIGHT => Self::Right,
            DOWN => Self::Down,
            LEFT => Self::Left,
            _ => None?,
        })
    }

    fn unit_vec(&self) -> Vec2<isize> {
        match self {
            Self::Up => Vec2 { x: 0, y: -1 },
            Self::Right => Vec2 { x: 1, y: 0 },
            Self::Down => Vec2 { x: 0, y: 1 },
            Self::Left => Vec2 { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
    Box,
}
impl Cell {
    fn new(symbol: char) -> Option<Self> {
        Some(match symbol {
            EMPTY => Self::Empty,
            WALL => Self::Wall,
            BOX => Self::Box,
            _ => None?,
        })
    }
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (mut cells, directions, mut robot_pos) = read_input(file);
    // println!("Initial state:");
    // display_cells(&cells, &robot_pos);
    // println!("directions\n{:?}", directions);
    // println!("robot_pos {:?}", robot_pos);

    for direction in directions.into_iter().map(|d| d.unit_vec()) {
        // println!(
        //     "\nMove {:?}",
        //     match direction {
        //         Vec2 { x: 0, y: -1 } => UP,
        //         Vec2 { x: 1, y: 0 } => RIGHT,
        //         Vec2 { x: 0, y: 1 } => DOWN,
        //         Vec2 { x: -1, y: 0 } => LEFT,
        //         _ => panic!(),
        //     }
        // );
        if let Some(move_index) = robot_pos.signed_add(direction) {
            if let Some(move_cell) = cells.get(move_index) {
                if *move_cell == Cell::Box {
                    let mut index = move_index.signed_add(direction);
                    while let Some(cell) = index.and_then(|i| cells.get(i)) {
                        match &cell {
                            Cell::Empty => {
                                cells[move_index] = Cell::Empty;
                                cells[index.unwrap()] = Cell::Box;
                                break;
                            }
                            Cell::Wall => break,
                            Cell::Box => index = index.unwrap().signed_add(direction),
                        }
                    }
                }
                if cells[move_index] == Cell::Empty {
                    robot_pos = move_index;
                }
            }
        }
        // display_cells(&cells, &robot_pos);
    }

    let gps = cells
        .indexed_iter()
        .filter_map(|((y, x), cell)| match cell {
            Cell::Box => Some(GPS_FACTOR * y + x),
            _ => None,
        })
        .sum::<usize>() as i64;

    Ok(gps)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;

    Ok(0)
}

fn read_input(file: File) -> (Array2<Cell>, Vec<Direction>, Vec2<usize>) {
    let mut robot_pos = None;
    let mut first_section = true;

    let mut cells = Vec::new();
    let mut directions = Vec::new();
    let mut rows = 0;
    for (row, line) in BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
    {
        if line.is_empty() {
            first_section = false
        } else if first_section {
            rows += 1;
            cells.extend(line.chars().enumerate().map(|(col, ch)| {
                if ch == ROBOT {
                    robot_pos = Some(Vec2 { x: col, y: row });
                }
                Cell::new(ch).unwrap_or(Cell::Empty)
            }));
        } else {
            directions.extend(line.chars().map(|ch| Direction::new(ch).unwrap()));
        }
    }
    let cols = cells.len() / rows;
    (
        Array2::from_shape_vec((rows, cols), cells).unwrap(),
        directions,
        robot_pos.unwrap(),
    )
}

fn display_cells(cells: &Array2<Cell>, robot_pos: &Vec2<usize>) {
    for (y, row) in cells.axis_iter(Axis(0)).enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!(
                "{}",
                if *robot_pos == (Vec2 { x, y }) {
                    ROBOT
                } else {
                    match cell {
                        Cell::Empty => EMPTY,
                        Cell::Wall => WALL,
                        Cell::Box => BOX,
                    }
                }
            )
        }
        println!();
    }
}
