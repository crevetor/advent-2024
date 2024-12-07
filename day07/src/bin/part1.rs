use anyhow::{anyhow, bail, Context, Result};
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Ops {
    Add,
    Mul,
}

impl Ops {
    fn apply(&self, x: i128, y: i128) -> i128 {
        match self {
            Ops::Add => x + y,
            Ops::Mul => x * y,
        }
    }
}

impl TryFrom<char> for Ops {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '0' => Ok(Ops::Add),
            '1' => Ok(Ops::Mul),
            _ => Err(anyhow!("Unknown op: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Operation {
    total: i128,
    numbers: Vec<i128>,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (total, numbers) = s.split_once(":").context("No ':' found")?;
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<i128>())
            .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;
        Ok(Operation {
            total: total.parse()?,
            numbers,
        })
    }
}

impl Operation {
    fn result(&self) -> Result<i128> {
        for i in 0..(2u32.pow(self.numbers.len() as u32)) {
            let ops = format!("{i:020b}")
                .chars()
                .skip(20 - self.numbers.len() + 1)
                .map(|c| Ops::try_from(c))
                .collect::<Result<Vec<_>>>()?;

            let mut total = 0;
            for (j, (a, b)) in self
                .numbers
                .iter()
                .zip(self.numbers.iter().skip(1))
                .enumerate()
            {
                if j == 0 {
                    total = ops[j].apply(*a, *b);
                    continue;
                }
                total = ops[j].apply(total, *b);
            }
            if total == self.total {
                println!("{self:?}, {ops:?}");
                return Ok(self.total);
            }
        }
        Ok(0)
    }
}

fn read_input(filename: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(filename)?;
    Ok(content.lines().map(|l| l.trim().to_string()).collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Wrong number of args");
    }

    let contents = read_input(&args[1])?;
    let operations = contents
        .iter()
        .map(|l| Operation::from_str(l))
        .collect::<Result<Vec<_>>>()?;
    let max_len = operations.iter().map(|op| op.numbers.len()).max().unwrap();
    println!("{max_len:?}");

    let mut sum: i128 = 0;
    for op in operations {
        sum += op.result()?;
    }
    println!("{sum}");

    Ok(())
}
