use anyhow::{Context, Result};
use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    empty: bool,
    visited: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perform_move(&self, coords: &[usize; 2]) -> [usize; 2] {
        match self {
            Direction::Up => [coords[0], coords[1] - 1],
            Direction::Down => [coords[0], coords[1] + 1],
            Direction::Left => [coords[0] - 1, coords[1]],
            Direction::Right => [coords[0] + 1, coords[1]],
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

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
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

    let mut pos = mat
        .find(Cell {
            empty: true,
            visited: true,
        })
        .context("Unable to find starting pos")?;
    let mut direction = Direction::Up;

    loop {
        let next_pos = direction.perform_move(&pos);
        if let Ok(next_cell) = mat.get_mut(next_pos[0], next_pos[1]) {
            if next_cell.empty {
                pos = next_pos;
                next_cell.visited = true;
            } else {
                direction = direction.perform_turn();
            }
        } else {
            break;
        }
    }

    let visited = mat.contents.iter().flatten().filter(|c| c.visited).count();
    println!("{visited}");
    Ok(())
}
