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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let reports: Vec<Vec<i64>> = contents
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| i64::from_str_radix(x, 10).unwrap())
                .collect()
        })
        .collect();

    let mut num_safe = 0;
    for report in reports {
        let mut safe = true;
        let mut prev_diff = report[1] - report[0];
        for (l1, l2) in report.iter().zip(report.iter().skip(1)) {
            let cur_diff = l2 - l1;
            if cur_diff == 0 || prev_diff.signum() != cur_diff.signum() || cur_diff.abs() > 3 {
                safe = false;
                break;
            }
            prev_diff = cur_diff;
        }
        if safe {
            num_safe += 1;
        }
    }

    println!("{}", num_safe);
}
