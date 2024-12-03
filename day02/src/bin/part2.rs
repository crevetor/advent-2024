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

fn is_safe(report: &[i64]) -> bool {
    let mut prev_diff = report[1] - report[0];
    for (l1, l2) in report.iter().zip(report.iter().skip(1)) {
        let cur_diff = l2 - l1;
        if cur_diff == 0 || prev_diff.signum() != cur_diff.signum() || cur_diff.abs() > 3 {
            return false;
        }
        prev_diff = cur_diff;
    }
    true
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
        if is_safe(&report) {
            num_safe += 1;
        } else {
            for i in 0..report.len() {
                if is_safe(
                    &report
                        .iter()
                        .enumerate()
                        .filter(|(n, level)| *n != i)
                        .map(|(n, level)| *level)
                        .collect::<Vec<i64>>(),
                ) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }

    println!("{}", num_safe);
}
