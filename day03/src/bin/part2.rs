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
    let mul_regex = Regex::new(r"(do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let mut total = 0;
    let mut process = true;
    for line in contents {
        for cap in mul_regex.captures_iter(&line) {
            if let Some(m) = cap.get(0) {
                match m.as_str().split_once('(').unwrap().0 {
                    "do" => process = true,
                    "don't" => process = false,
                    "mul" => {
                        if process {
                            let left = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                            let right = cap.get(3).unwrap().as_str().parse::<u64>().unwrap();
                            println!("{left}*{right}");
                            total += left * right;
                        }
                    },
                    _ => (),

                }
            }
        }
    }
    println!("Total: {}", total);
    Ok(())
}