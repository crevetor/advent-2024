use anyhow::{Context, Result};
use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perform_move(&self, coords: &[usize; 2]) -> Result<[usize; 2]> {
        match self {
            Direction::Up => Ok([
                coords[0],
                coords[1]
                    .checked_sub(1)
                    .context("Tried to substract from 0")?,
            ]),
            Direction::Down => Ok([coords[0], coords[1] + 1]),
            Direction::Left => Ok([
                coords[0]
                    .checked_sub(1)
                    .context("Tried to substract from 0")?,
                coords[1],
            ]),
            Direction::Right => Ok([coords[0] + 1, coords[1]]),
        }
    }

    fn perform_turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Cell {
    empty: bool,
    visited: bool,
}
fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn test_for_loop(start_pos: [usize; 2], cells: &mut Matrix<Cell>) -> bool {
    let mut direction = Direction::Up;
    let mut pos = start_pos;
    let mut already_visited = 0;
    let max_visited = cells.contents.iter().flatten().count();

    loop {
        let next_pos = direction.perform_move(&pos);
        if next_pos.is_err() {
            return false;
        }
        let next_pos = next_pos.unwrap();
        if let Ok(next_cell) = cells.get_mut(next_pos[0], next_pos[1]) {
            if next_cell.empty {
                if next_cell.visited {
                    already_visited += 1;
                    if already_visited >= max_visited {
                        return true;
                    }
                }
                pos = next_pos;
                next_cell.visited = true;
            } else {
                direction = direction.perform_turn();
            }
        } else {
            return false;
        }
    }
}

fn print_mat(mat: &Matrix<Cell>) {
    for row in mat.rows() {
        println!(
            "{}",
            row.iter()
                .map(|c| if c.empty {
                    if c.visited {
                        '^'
                    } else {
                        '.'
                    }
                } else {
                    '#'
                })
                .collect::<String>()
        );
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut mat: Matrix<Cell> = contents
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        Cell {
                            empty: false,
                            visited: false,
                        }
                    } else if c == '.' {
                        Cell {
                            empty: true,
                            visited: false,
                        }
                    } else {
                        Cell {
                            empty: true,
                            visited: true,
                        }
                    }
                })
                .collect()
        })
        .collect();
    let pos = mat
        .find(Cell {
            empty: true,
            visited: true,
        })
        .context("Unable to find starting pos")?;

    let mut num_loops = 0;
    for y in 0..mat.num_rows() {
        for x in 0..mat.num_cols() {
            if let Ok(Cell {
                empty: true,
                visited: false,
            }) = mat.get(x, y)
            {
                let mut newmat = mat.clone();
                newmat.get_mut(x, y)?.empty = false;
                if test_for_loop(pos, &mut newmat) {
                    num_loops += 1;
                }
            }
        }
    }
    println!("Number of loops: {}", num_loops);

    Ok(())
}
