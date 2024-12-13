use anyhow::{bail, Result};
use matrix::Matrix;
use std::env;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().map(|l| l.trim().to_string()).collect())
}

fn follow_trail(
    mat: &Matrix<usize>,
    pos: [usize; 2],
    visited: &mut Vec<[usize; 2]>,
) -> Result<Vec<Vec<[usize; 2]>>> {
    visited.push(pos);
    let val = mat.get(pos[0], pos[1])?;

    if val == 9 {
        return Ok(vec![visited.clone()]);
    }

    let mut ret = Vec::new();

    for neighbor in mat.get_neighbors(pos[0], pos[1]) {
        if neighbor.1 == val + 1 {
            ret.extend(follow_trail(mat, neighbor.0, visited)?);
        }
    }

    Ok(ret)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let mat = Matrix::from_iter(contents.iter().map(|line| {
        line.chars()
            .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
            .collect()
    }));

    let heads = mat.find_all(0)?;
    let scores: Vec<Vec<Vec<[usize; 2]>>> = heads
        .iter()
        .filter_map(|h| follow_trail(&mat, *h, &mut vec![]).ok())
        .collect();

    let final_score: usize = scores.iter().map(|h| h.len()).sum();
    println!(
        "{:?}",
        scores.iter().map(|s| s.len()).collect::<Vec<usize>>()
    );
    println!("{final_score}");

    Ok(())
}
