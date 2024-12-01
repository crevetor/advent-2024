use std::env;
use std::fs;
use std::io::BufRead;
use std::process;

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        left.push(i64::from_str_radix(cols[0], 10).unwrap());
        right.push(i64::from_str_radix(cols[1], 10).unwrap());
    }
    left.sort();
    right.sort();

    let mut distances = Vec::new();
    for (l, r) in left.iter().zip(right.iter()) {
        distances.push((l - r).abs());
    }
    println!("{}", distances.iter().sum::<i64>());
}