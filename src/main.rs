// use itertools::Itertools;
// use std::collections::{BTreeMap, HashMap};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::ops::{Add, Sub};

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/09.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let str_input = fs::read_to_string(path)?;

    let (mut h_x, mut h_y) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for line in reader.lines() {
        let instruction = line?;
        let (direction, steps_count) = instruction.split_at(1);
        mv(
            &mut h_x,
            &mut h_y,
            direction,
            steps_count.trim().parse::<i32>().unwrap(),
        );
        for i in 0..steps_count.trim().parse::<i32>().unwrap() {
            
        }

        println!("{} {}", h_x, h_y)
    }

    Ok(println!("{:?}", visited))
}

fn mv(x: &mut i32, y: &mut i32, direction: &str, steps: i32) {
    match direction {
        "R" => *x += steps,
        "L" => *x -= steps,
        "D" => *y -= steps,
        "U" => *y += steps,
        _ => (),
    }
}

fn catch_up(h_x: &i32, h_y: &i32, t_x: &mut i32, t_y: &mut i32) {
    if (h_x - *t_x).abs() + (h_y - *t_y).abs() <= 1 {
        println!("stuck {} {} / {} {}", h_x, h_y, t_x, t_y);
    }
}
