use anyhow::Result;
use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

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

    for y in 0..mat.num_rows() {
        let line = mat.row(y)?.iter().collect::<String>();
        let diag = mat.diag([0, y], [1, 1])?.iter().collect::<String>();
        let other_diag = mat
            .diag([mat.num_cols() - 1, y], [-1, 1])?
            .iter()
            .collect::<String>();
        println!("row {y}");
        println!("{line:?}");
        println!("{diag:?}");
        println!("{other_diag:?}");
        num_xmas += line.matches("XMAS").count();
        num_xmas += diag.matches("XMAS").count();
        num_xmas += other_diag.matches("XMAS").count();
        num_xmas += line
            .chars()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .count();
        num_xmas += diag
            .chars()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .count();
        num_xmas += other_diag
            .chars()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .count();
    }
    for x in 0..mat.num_cols() {
        let col = mat.col(x)?.iter().collect::<String>();
        let diag = mat.diag([x, 0], [1, 1])?.iter().collect::<String>();
        let other_diag = mat.diag([x, 0], [-1, 1])?.iter().collect::<String>();
        println!("col {x}");
        println!("{col:?}");
        println!("{diag:?}");
        println!("{other_diag:?}");
        num_xmas += col.matches("XMAS").count();
        num_xmas += col
            .chars()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .count();
        if x > 0 && x < mat.num_cols() - 1 {
            num_xmas += diag.matches("XMAS").count();
            num_xmas += other_diag.matches("XMAS").count();
            num_xmas += diag
                .chars()
                .rev()
                .collect::<String>()
                .matches("XMAS")
                .count();
            num_xmas += other_diag
                .chars()
                .rev()
                .collect::<String>()
                .matches("XMAS")
                .count();
        }
    }
    println!("{}", num_xmas);
    Ok(())
}
