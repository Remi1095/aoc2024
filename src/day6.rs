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

#[derive(Clone, Debug)]
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
    if !walk_guard(&mut cells, &mut guard, |_| visited += 1) {
        panic!("Guard walking in cycle");
    }
    Ok(visited)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (cells, guard) = read_input(file);
    let mut visited = Vec::new();
    if !walk_guard(&mut cells.clone(), &mut guard.clone(), |guard| {
        visited.push(guard.get_position_index().unwrap())
    }) {
        panic!("Guard walking in cycle");
    };
    let loops = visited
        .into_iter()
        .filter(|idx| {
            let mut cells_clone = cells.clone();
            cells_clone[*idx] = Cell::Obstacle;
            !walk_guard(&mut cells_clone, &mut guard.clone(), |_| {})
        })
        .count() as i64;

    Ok(loops)
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

fn walk_guard<F>(cells: &mut Array2<Cell>, guard: &mut Guard, mut predicate: F) -> bool
where
    F: FnMut(&Guard) -> (),
{
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
                predicate(&guard);
            }
            Cell::Visited(prev_direction) if *prev_direction == guard.direction => {
                return false;
            }
            _ => {}
        }
        guard.move_forward(1);
    }
    true
}