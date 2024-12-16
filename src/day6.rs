use crate::{get_text_file, SolutionResult};
use ndarray::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/6/input";

const EMPTY: char = '.';
const OBSTACTLE: char = '#';
const GUARD_UP: char = '^';
const GUARD_DOWN: char = 'v';
const GUARD_RIGHT: char = '>';
const GUARD_LEFT: char = '<';

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cell {
    Obstacle,
    Empty,
    Visited(Direction),
}

#[derive(Debug)]
struct Guard {
    // (row, col)
    // (x, y)
    position: (isize, isize),
    direction: Direction,
}
impl Guard {
    fn move_forward(&mut self, step: isize) {
        match self.direction {
            Direction::Up => self.position.0 -= step,
            Direction::Right => self.position.1 += step,
            Direction::Down => self.position.0 += step,
            Direction::Left => self.position.1 -= step,
        }
    }

    fn get_position_index(&self) -> Result<(usize, usize), Box<dyn Error>> {
        Ok((self.position.0.try_into()?, self.position.1.try_into()?))
    }
}

impl Direction {
    fn new(symbol: char) -> Option<Self> {
        Some(match symbol {
            GUARD_UP => Self::Up,
            GUARD_RIGHT => Self::Right,
            GUARD_DOWN => Self::Down,
            GUARD_LEFT => Self::Left,
            _ => None?,
        })
    }

    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (mut cells, mut guard) = read_input(file);
    let mut visited = 0;
    while let Some(cell) = {
        if let Ok(idx) = guard.get_position_index() {
            cells.get_mut(idx)
        } else {
            None
        }
    } {
        match cell {
            Cell::Obstacle => {
                guard.move_forward(-1);
                guard.direction = guard.direction.rotate();
            }
            Cell::Empty => {
                *cell = Cell::Visited(guard.direction.clone());
                visited += 1;
            }
            Cell::Visited(prev_direction) if *prev_direction == guard.direction => {
                panic!("Guard stuck in loop")
            }
            _ => {}
        }
        guard.move_forward(1);
    }

    Ok(visited)
}

pub fn part_2() -> SolutionResult {
    Ok(0)
}

fn read_input(file: File) -> (Array2<Cell>, Guard) {
    let mut cells = Vec::new();
    let mut guard = None;

    let rows = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(row, line)| {
            cells.extend(line.unwrap().chars().enumerate().map(|(col, ch)| match ch {
                EMPTY => Cell::Empty,
                OBSTACTLE => Cell::Obstacle,
                _ => {
                    if let (Some(direction), None) = (Direction::new(ch), &guard) {
                        guard = Some(Guard {
                            position: (row as isize, col as isize),
                            direction: direction.clone(),
                        });
                        Cell::Empty
                    } else {
                        panic!("Invalid input symbol")
                    }
                }
            }))
        })
        .count();

    let cols = cells.len() / rows;
    (
        Array2::from_shape_vec((rows, cols), cells).unwrap(),
        guard.unwrap(),
    )
}

fn display_cells(cells: &Array2<Cell>, guard: &Guard) {
    for (x, row) in cells.axis_iter(Axis(0)).enumerate() {
        for (y, cell) in row.iter().enumerate() {
            print!(
                "{}",
                if guard.position == (x as isize, y as isize) {
                    match guard.direction {
                        Direction::Up => GUARD_UP,
                        Direction::Right => GUARD_RIGHT,
                        Direction::Down => GUARD_DOWN,
                        Direction::Left => GUARD_LEFT,
                    }
                } else {
                    match cell {
                        Cell::Obstacle => OBSTACTLE,
                        Cell::Empty => EMPTY,
                        Cell::Visited(_) => 'X',
                    }
                }
            )
        }
        println!();
    }
}
