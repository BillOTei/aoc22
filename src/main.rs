// use itertools::Itertools;
// use std::collections::{BTreeMap, HashMap};
use pathfinding::dijkstra;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32);

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/12.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let str_input = fs::read_to_string(path)?;
    let grid: Vec<Vec<Pos>> = reader
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .map(|(x, number)| Pos(x as i32, y as i32, number as i32))
                .collect()
        })
        .collect();

    impl Pos {
        fn neighbours(&self, g: &Vec<Vec<Pos>>) -> Vec<(Pos, i32)> {
            let &Pos(x, y, h) = self;
            let right = g.get(y as usize).and_then(|row| row.get(x as usize + 1));
            let left = if x - 1 >= 0 {
                g.get(y as usize).and_then(|row| row.get(x as usize - 1))
            } else {
                None
            };
            let up = if y - 1 >= 0 {
                g.get(y as usize - 1).and_then(|row| row.get(x as usize))
            } else {
                None
            };
            let down = g.get(y as usize + 1).and_then(|row| row.get(x as usize));

            println!("{:?} {:?} {:?}", x, y, h);

            vec![right, left, up, down]
                .into_iter()
                .filter(|maybe_pos| maybe_pos.is_some() && maybe_pos.unwrap().2 <= h + 1)
                .map(|p| (p.unwrap().clone(), p.unwrap().2))
                .collect()
        }
    }

    let result = dijkstra(&Pos(0, 0, 97), |p| p.neighbours(&grid), |p| p.2 == 69 && p);

    println!("{:?}", grid);

    Ok(println!("{:?}", result))
}
