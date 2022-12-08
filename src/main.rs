// use itertools::Itertools;
// use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

use itertools::Itertools;

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/08.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let str_input = fs::read_to_string(path)?;

    let trees_map: Vec<Vec<u32>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let x_len = &trees_map.first().unwrap().len() - 1;
    let y_len = &trees_map.len() - 1;
    let inverted_trees_map: Vec<Vec<u32>> = transpose(trees_map.clone());

    let mut count: u32 = 0;

    for (y, row) in trees_map.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if x == 0 || x == x_len || y == 0 || y == y_len {
            } else {
                let temp = visible_count(x, y, t, row, inverted_trees_map.get(x).unwrap());
                if temp > count {
                    count = temp
                }
            }
        }
    }

    Ok(println!("{:?}", count))
}

// fn is_visible(x: usize, y: usize, height: &u32, row: &Vec<u32>, col: &Vec<u32>) -> bool {
//     let left = row[..x].into_iter().all(|h| h < height).to_owned();
//     let right = row[x + 1..].into_iter().all(|h| h < height).to_owned();
//     let up = col[..y].into_iter().all(|h| h < height).to_owned();
//     let down = col[y + 1..].into_iter().all(|h| h < height).to_owned();

//     up || down || left || right
// }

fn visible_count(x: usize, y: usize, height: &u32, row: &Vec<u32>, col: &Vec<u32>) -> u32 {
    let left = &row[..x].into_iter().rev().collect_vec();
    let mut left_count = 0;
    for h in left {
        left_count += 1;
        if h >= &height {
            break;
        }
    }

    let right = &row[x + 1..];
    let mut right_count = 0;
    for h in right {
        right_count += 1;
        if h >= height {
            break;
        }
    }

    let up = &col[..y].into_iter().rev().collect_vec();
    let mut up_count = 0;
    for h in up {
        up_count += 1;
        if h >= &height {
            break;
        }
    }

    let down = &col[y + 1..];
    let mut down_count = 0;
    for h in down {
        down_count += 1;
        if h >= height {
            break;
        }
    }

    (left_count * right_count * up_count * down_count) as u32
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
