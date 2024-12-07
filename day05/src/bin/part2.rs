use anyhow::{bail, Context, Result};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct Order {
    first: usize,
    second: usize,
}

impl Order {
    fn new(order: &str) -> Result<Self> {
        let (left, right) = order.split_once('|').context("Couldn't split order")?;

        Ok(Order {
            first: left.parse()?,
            second: right.parse()?,
        })
    }
}
struct OrderChain {
    orders: HashMap<usize, Vec<usize>>,
}

impl From<HashMap<usize, Vec<usize>>> for OrderChain {
    fn from(value: HashMap<usize, Vec<usize>>) -> Self {
        Self { orders: value }
    }
}

impl OrderChain {
    fn new() -> Self {
        OrderChain {
            orders: HashMap::new(),
        }
    }

    fn add(&mut self, order: &str) -> Result<()> {
        let order = Order::new(order)?;
        if self.orders.contains_key(&order.first) {
            self.orders
                .get_mut(&order.first)
                .unwrap()
                .push(order.second);
        } else {
            self.orders.insert(order.first, vec![order.second]);
        }
        if !self.orders.contains_key(&order.second) {
            self.orders.insert(order.second, Vec::new());
        }

        Ok(())
    }

    fn head(&self) -> Result<usize> {
        let left_set: HashSet<usize> = HashSet::from_iter(self.orders.keys().cloned());
        let right_set: HashSet<usize> = HashSet::from_iter(self.orders.values().flatten().cloned());

        let mut heads = left_set.difference(&right_set).collect::<Vec<_>>();
        if heads.is_empty() || heads.len() > 1 {
            bail!("Must have only one head {:?} {:?}", heads, self.orders);
        }

        Ok(*heads[0])
    }

    fn chains(&self, head: usize, chains: &[usize]) -> Vec<Vec<usize>> {
        let new_chains = [chains, &vec![head]].concat();
        let mut ret = Vec::new();
        if !self.orders.contains_key(&head) || self.orders[&head].is_empty() {
            return vec![new_chains];
        }
        for neighbor in self.orders[&head].iter() {
            ret.extend(self.chains(*neighbor, &new_chains));
        }

        ret
    }

    fn as_list(&self) -> Result<Vec<usize>> {
        let head = self.head()?;

        let chains = self.chains(head, &vec![]);

        let mut longest_chain = &chains[0];
        for chain in chains.iter() {
            if chain.len() > longest_chain.len() {
                longest_chain = chain;
            }
        }

        Ok(longest_chain.clone())
    }

    fn filtered_chain(&self, pages: &[usize]) -> Self {
        Self::from(
            self.orders
                .iter()
                .filter(|(k, v)| pages.contains(k))
                .map(|(k, v)| (*k, v.clone()))
                .collect::<HashMap<_, _>>(),
        )
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

fn reorder(chain_list: &[usize], pages: &mut [usize]) -> bool {
    let mut reorder = false;
    let mut rev_chain = chain_list
        .iter()
        .filter(|x| pages.contains(x))
        .rev()
        .collect::<Vec<_>>();
    let mut current_smallest = rev_chain.pop().unwrap();
    for i in 0..pages.len() {
        if rev_chain.len() == 0 {
            break;
        }
        if &pages[i] == current_smallest {
            current_smallest = rev_chain.pop().unwrap();
            continue;
        }

        if &pages[i] != current_smallest && chain_list.contains(&pages[i]) {
            if let Some(smallest_idx) = pages.iter().position(|c| c == current_smallest) {
                pages.swap(i, smallest_idx);
                current_smallest = rev_chain.pop().unwrap();
                reorder = true;
                continue;
            }
        }
    }
    reorder
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut order_chain = OrderChain::new();
    let mut content_iter = contents.iter();
    while let Some(line) = content_iter.next() {
        if line.trim().is_empty() {
            break;
        }
        order_chain.add(line)?;
    }
    //let chain_list = order_chain.as_list()?;
    //println!("{:?}", chain_list);

    let mut sum = 0;
    let mut updates: Vec<Vec<usize>> = content_iter
        .map(|line| line.trim().split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    for update in &mut updates {
        println!("Update: {:?}", update);
        let filtered = order_chain.filtered_chain(update);
        let reordered = reorder(&filtered.as_list()?, update);
        if reordered {
            sum += update[update.len() / 2];
        }
    }
    println!("{sum}");
    Ok(())
}
