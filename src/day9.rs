use crate::{get_text_file, SolutionResult};
use std::{error::Error, fs::File, io::Read};

const INPUT_URL: &str = "https://adventofcode.com/2024/day/9/input";

#[derive(Clone, Debug)]
struct Block {
    size: usize,
    offset: usize,
}

pub fn part_1() -> SolutionResult {
    let file = get_text_file(INPUT_URL)?;
    let (used_blocks, free_blocks) = read_input(file)?;

    let mut used_iter = used_blocks.into_iter().enumerate().rev().peekable();
    let mut free_iter = free_blocks.into_iter().peekable();
    // let mut used_it = used_iter.next();
    // let mut free_it = free_iter.next();

    let mut checksum = 0;

    while {
        used_iter
            .peek()
            .map(|(_, used)| {
                free_iter
                    .peek()
                    .map(|free| used.offset > free.offset)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    } {
        let (id, used) = used_iter.peek_mut().unwrap();
        let free = free_iter.peek_mut().unwrap();

        // println!();
        // println!("id {:?}", id);
        // println!("used {:?}", used);
        // println!("free {:?}", free);

        let used_size_prev = used.size;
        used.size = used.size.saturating_sub(free.size);

        let blocks_moved = used_size_prev - used.size;
        // println!(
        //     "blocks_moved {} first {} last {}",
        //     blocks_moved,
        //     free.offset,
        //     free.offset + blocks_moved - 1
        // );
        let t = arithmetic_series(blocks_moved, free.offset, free.offset + blocks_moved - 1) * *id;
        // println!("checksum {:?}", t);
        checksum += t;

        free.size -= blocks_moved;
        free.offset += blocks_moved;

        // println!("new used {:?}", used);
        // println!("new free {:?}", free);

        if used.size == 0 {
            used_iter.next();
        }
        if free.size == 0 {
            free_iter.next();
        }
    }
    for (id, used) in used_iter {
        // println!();
        // println!("id {:?}", id);
        // println!("used {:?}", used);
        let t = arithmetic_series(used.size, used.offset, used.offset + used.size - 1) * id;
        // println!("checksum {:?}", t);
        checksum += t;
    }

    Ok(checksum as i64)
}

pub fn part_2() -> SolutionResult {
    Ok(0)
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
            if let Some(size) =  ch.to_digit(10) {
                let size = size as usize;
                offset += size;
                Some(Block {
                    size,
                    offset: offset_clone,
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
