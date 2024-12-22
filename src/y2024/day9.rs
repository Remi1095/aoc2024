use crate::{get_text_file, SolutionResult};
use std::{error::Error, fs::File, io::Read};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/9/input";

#[derive(Clone, Debug)]
struct Block {
    offset: usize,
    size: usize,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (used_blocks, free_blocks) = read_input(file)?;

    let mut used_iter = used_blocks.into_iter().enumerate().rev().peekable();
    let mut free_iter = free_blocks.into_iter().peekable();

    let mut checksum = 0;

    while match (used_iter.peek(), free_iter.peek()) {
        (Some((_, used)), Some(free)) => used.offset > free.offset,
        _ => false,
    } {
        let (id, used) = used_iter.peek_mut().unwrap();
        let free = free_iter.peek_mut().unwrap();

        let used_size_prev = used.size;
        used.size = used.size.saturating_sub(free.size);

        let blocks_moved = used_size_prev - used.size;
        checksum +=
            arithmetic_series(blocks_moved, free.offset, free.offset + blocks_moved - 1) * *id;

        free.size -= blocks_moved;
        free.offset += blocks_moved;

        if used.size == 0 {
            used_iter.next();
        }
        if free.size == 0 {
            free_iter.next();
        }
    }
    for (id, used) in used_iter {
        checksum += arithmetic_series(used.size, used.offset, used.offset + used.size - 1) * id;
    }

    Ok(checksum as i64)
}

pub fn part_2() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (used_blocks, mut free_blocks) = read_input(file)?;

    let checksum = used_blocks
        .into_iter()
        .enumerate()
        .rev()
        .map(|(used_id, used)| {
            free_blocks
                .iter_mut()
                .try_for_each(|free| {
                    if free.offset > used.offset {
                        Err(None)
                    } else if used.size <= free.size {
                        Err(Some(free))
                    } else {
                        Ok(())
                    }
                })
                .err()
                .flatten()
                .map(|free| {
                    let checksum =
                        arithmetic_series(used.size, free.offset, free.offset + used.size - 1)
                            * used_id;
                    free.size -= used.size;
                    free.offset += used.size;
                    checksum
                })
                .unwrap_or(
                    arithmetic_series(used.size, used.offset, used.offset + used.size - 1)
                        * used_id,
                )
        })
        .sum::<usize>() as i64;

    Ok(checksum)
}

fn read_input(mut file: File) -> Result<(Vec<Block>, Vec<Block>), Box<dyn Error>> {
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let mut left_side: bool = false;
    let mut offset: usize = 0;
    Ok(text
        .chars()
        .filter_map(|ch| {
            let offset_clone = offset;
            if let Some(size) = ch.to_digit(10) {
                let size = size as usize;
                offset += size;
                Some(Block {
                    offset: offset_clone,
                    size,
                })
            } else {
                None
            }
        })
        .partition(|_| {
            left_side = !left_side;
            left_side
        }))
}

fn arithmetic_series(size: usize, first: usize, last: usize) -> usize {
    size * (first + last) / 2
}
