use std::env;
use std::fs;
use std::process;
use matrix::Matrix;
use anyhow::Result;

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
    let mat = Matrix::new(contents.iter().map(|s| s.chars().collect()).collect());
    let mut num_xmas = 0;

    for y in 1..mat.num_rows()-1 {
        for x in 1..mat.num_cols()-1 {
            let sub = mat.get_sub_matrix(x, y, [3,3])?;
            let diag = sub.diag([0, 0], [1, 1])?;
            let other_diag = sub.diag([2, 0], [-1, 1])?;
            if (diag.iter().collect::<String>() == "MAS" || diag.iter().rev().collect::<String>() == "MAS") && (other_diag.iter().collect::<String>() == "MAS" || other_diag.iter().rev().collect::<String>() == "MAS") {
                num_xmas += 1;
            }
        }
    }
    println!("{num_xmas}");

    Ok(())
}