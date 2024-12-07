use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct Order {
    first: isize,
    second: isize,
}

impl Order {
    fn new(order_str: &str) -> Result<Self> {
        let (left, right) = order_str.split_once('|').context("Unable to split order")?;

        Ok(Order {
            first: left.parse()?,
            second: right.parse()?,
        })
    }

    fn is_respected(&self, updates: &[isize]) -> bool {
        if updates.contains(&self.first) && updates.contains(&self.second) {
            let mut first_found = false;
            for num in updates {
                if *num == self.first {
                    first_found = true;
                }
                if *num == self.second {
                    if first_found {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
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
    let mut contents_iter = contents.iter();
    let mut orders = Vec::new();
    while let Some(line) = contents_iter.next() {
        if line.trim().is_empty() {
            break;
        }
        orders.push(Order::new(line)?);
    }
    let updates: Vec<Vec<isize>> = contents_iter
        .map(|line| line.trim().split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    let mut sum = 0;
    for update in updates.iter() {
        let orders_respected = orders.iter().map(|x| x.is_respected(update)).all(|x| x);
        if orders_respected {
            if update.len() % 2 == 0 {
                bail!("Update has even number of elements.");
            }
            sum += update[update.len() / 2];
        }
    }
    println!("{}", sum);
    Ok(())
}
