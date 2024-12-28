use crate::{get_text_file, math::Vec2, SolutionResult};
use ndarray::Array2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

#[derive(Clone, PartialEq, Eq, Debug)]
enum BoxCell {
    Left,
    Right,
}
impl BoxCell {
    fn indices(&self, index: Vec2<usize>) -> (Vec2<usize>, Vec2<usize>) {
        match self {
            BoxCell::Left => (
                index,
                Vec2 {
                    x: index.x + 1,
                    ..index
                },
            ),
            BoxCell::Right => (
                Vec2 {
                    x: index.x - 1,
                    ..index
                },
                index,
            ),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum WideCell {
    Empty,
    Wall,
    Box(BoxCell),
}
impl WideCell {
    fn from_cell(cell: Cell) -> [Self; 2] {
        match cell {
            Cell::Empty => [Self::Empty, Self::Empty],
            Cell::Wall => [Self::Wall, Self::Wall],
            Cell::Box => [Self::Box(BoxCell::Left), Self::Box(BoxCell::Right)],
        }
    }

    fn unwrap_box(self) -> BoxCell {
        match self {
            Self::Box(b) => b,
            _ => panic!(),
        }
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
    let (cells, directions, robot_pos) = read_input(file);
    let (rows, cols) = cells.dim();
    let mut cells = Array2::from_shape_vec(
        (rows, cols * 2),
        cells
            .as_standard_layout()
            .into_iter()
            .flat_map(|cell| WideCell::from_cell(cell))
            .collect(),
    )
    .unwrap();
    let mut robot_pos = Vec2 {
        x: robot_pos.x * 2,
        ..robot_pos
    };

    // println!("Initial state");
    // display_wide_cells(&cells, &robot_pos);
    for direction in directions {
        // println!(
        //     "\nMove {:?}",
        //     match direction {
        //         Direction::Up => UP,
        //         Direction::Right => RIGHT,
        //         Direction::Down => DOWN,
        //         Direction::Left => LEFT,
        //     }.to_string()
        // );
        let unit_direction = direction.unit_vec();
        if let Some(move_index) = robot_pos.signed_add(unit_direction) {
            if match cells.get(move_index) {
                Some(WideCell::Empty) => true,
                Some(WideCell::Wall) | None => false,
                Some(WideCell::Box(_)) => match direction {
                    Direction::Up | Direction::Down => {
                        move_boxes_vertical(&mut cells, move_index, unit_direction)
                    }
                    Direction::Right | Direction::Left => {
                        move_boxes_horizontal(&mut cells, move_index, unit_direction)
                    }
                },
            } {
                robot_pos = move_index;
            }
        }
        // display_wide_cells(&cells, &robot_pos);
    }
    let gps = cells
        .indexed_iter()
        .filter_map(|((y, x), cell)| match cell {
            WideCell::Box(BoxCell::Left) => Some(GPS_FACTOR * y + x),
            _ => None,
        })
        .sum::<usize>() as i64;

    Ok(gps)
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

fn move_boxes_horizontal(
    cells: &mut Array2<WideCell>,
    move_index: Vec2<usize>,
    unit_direction: Vec2<isize>,
) -> bool {
    let mut box_indicies = vec![move_index];
    let mut index = 0;
    while let Some(box_idx) = box_indicies.get(index) {
        let next_box_idx_opt = box_idx.signed_add(unit_direction);
        match next_box_idx_opt.and_then(|i| cells.get(i)) {
            Some(WideCell::Empty) => break,
            Some(WideCell::Wall) | None => return false,
            Some(WideCell::Box(_)) => box_indicies.push(next_box_idx_opt.unwrap()),
        }
        index += 1;
    }
    for box_idx in box_indicies.iter().rev() {
        cells[box_idx.signed_add(unit_direction).unwrap()] = cells[*box_idx].clone();
    }
    cells[*box_indicies.first().unwrap()] = WideCell::Empty;
    true
}

fn move_boxes_vertical(
    cells: &mut Array2<WideCell>,
    move_index: Vec2<usize>,
    unit_direction: Vec2<isize>,
) -> bool {
    let move_box_cell = cells[move_index].clone().unwrap_box();
    let mut boxes = vec![move_box_cell.indices(move_index)];
    let mut next_box_idx = 0;

    let push_box = |box_side_idx: Vec2<usize>| {
        if let Some(next_index) = box_side_idx.signed_add(unit_direction) {
            match cells.get(next_index) {
                Some(WideCell::Wall) | None => None,
                Some(WideCell::Empty) => Some(None),
                Some(WideCell::Box(box_cell)) => Some(Some(box_cell.indices(next_index))),
            }
        } else {
            None
        }
    };

    while let Some((left_side, right_side)) = boxes.get(next_box_idx) {
        if let (Some(next_left_box_opt), Some(next_right_box_opt)) =
            (push_box(*left_side), push_box(*right_side))
        {
            if let Some(next_left_box) = next_left_box_opt {
                boxes.push(next_left_box);
            }
            if let Some(next_right_box) = next_right_box_opt {
                if boxes.last().map_or(true, |b| *b != next_right_box) {
                    boxes.push(next_right_box);
                }
            }
        } else {
            return false;
        }
        next_box_idx += 1;
    }

    for (left_side, right_side) in boxes.iter().rev() {
        cells[*left_side] = WideCell::Empty;
        cells[*right_side] = WideCell::Empty;
        cells[left_side.signed_add(unit_direction).unwrap()] = WideCell::Box(BoxCell::Left);
        cells[right_side.signed_add(unit_direction).unwrap()] = WideCell::Box(BoxCell::Right);
    }
    true
}

// fn display_cells(cells: &Array2<Cell>, robot_pos: &Vec2<usize>) {
//     for (y, row) in cells.axis_iter(Axis(0)).enumerate() {
//         for (x, cell) in row.iter().enumerate() {
//             print!(
//                 "{}",
//                 if *robot_pos == (Vec2 { x, y }) {
//                     ROBOT
//                 } else {
//                     match cell {
//                         Cell::Empty => EMPTY,
//                         Cell::Wall => WALL,
//                         Cell::Box => BOX,
//                     }
//                 }
//                 .to_string()
//             )
//         }
//         println!();
//     }
// }

// fn display_wide_cells(cells: &Array2<WideCell>, robot_pos: &Vec2<usize>) {
//     for (y, row) in cells.axis_iter(Axis(0)).enumerate() {
//         for (x, cell) in row.iter().enumerate() {
//             print!(
//                 "{}",
//                 if *robot_pos == (Vec2 { x, y }) {
//                     ROBOT
//                 } else {
//                     match cell {
//                         WideCell::Empty => EMPTY,
//                         WideCell::Wall => WALL,
//                         WideCell::Box(BoxCell::Left) => '[',
//                         WideCell::Box(BoxCell::Right) => ']',
//                     }
//                 }
//                 .to_string()
//             )
//         }
//         println!();
//     }
// }
