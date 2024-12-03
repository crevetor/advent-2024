use std::env;
use std::fs;
use std::process;
use regex::Regex;

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total = 0;
    for line in contents {
        for (_, [left, right]) in mul_regex.captures_iter(&line).map(|c| c.extract()) {
            let res = left.parse::<u64>()? * right.parse::<u64>()?;
            println!("{left}* {right} = {res}");
            total += res;
        }
    }
    println!("Total: {}", total);
    Ok(())
}
