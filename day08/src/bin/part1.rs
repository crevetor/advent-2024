use anyhow::{bail, Context, Result};
use itertools::Itertools;
use macroquad::prelude::{IVec2, Rect, Vec2};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct AntennaArray {
    id: char,
    antenna_pos: Vec<IVec2>,
}

impl AntennaArray {
    fn new(id: char) -> AntennaArray {
        AntennaArray {
            id,
            antenna_pos: Vec::new(),
        }
    }

    fn add_antenna(&mut self, antenna_pos: IVec2) {
        self.antenna_pos.push(antenna_pos);
    }

    fn antinodes(&self) -> Result<Vec<IVec2>> {
        let mut ret = Vec::new();
        for pair in self.antenna_pos.iter().combinations(2) {
            let pair_dist = *pair[0] - *pair[1];
            ret.push(*pair[0] + pair_dist);
            ret.push(*pair[1] - pair_dist);
        }
        Ok(ret)
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

    let mut antenna_arrays = HashMap::new();
    for (y, line) in contents.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                if !antenna_arrays.contains_key(&ch) {
                    antenna_arrays.insert(ch, AntennaArray::new(ch));
                }
                antenna_arrays
                    .get_mut(&ch)
                    .context("could not get antenna array")?
                    .add_antenna(IVec2::new(x as i32, y as i32));
            }
        }
    }

    antenna_arrays.values().for_each(|a| println!("{:?}", a));

    let antenna_positions: Vec<IVec2> =
        antenna_arrays.values().fold(Vec::new(), |mut acc, array| {
            acc.extend(array.antenna_pos.iter());
            acc
        });

    let mut antinodes = Vec::new();
    for array in antenna_arrays.values() {
        println!("{}", array.id);
        antinodes.extend(array.antinodes()?);
    }

    let bounds = Rect::new(0., 0., contents[0].len() as f32, contents.len() as f32);
    println!("{bounds:?}");

    let filter_antinodes = antinodes
        .iter()
        //.filter(|a| !antenna_positions.contains(a) && bounds.contains(a.as_vec2()))
        .filter(|a| bounds.contains(a.as_vec2()))
        .sorted_by(|a, b| {
            if a.x == b.x {
                a.y.cmp(&b.y)
            } else {
                a.x.cmp(&b.x)
            }
        })
        .dedup()
        .collect::<Vec<_>>();
    println!("{:?}", filter_antinodes);
    println!("{}", filter_antinodes.len());

    Ok(())
}
