use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("/home/billo/aoc22/src/puzzles/01.txt")?;
    let reader = BufReader::new(file);
    let mut r: u32 = 0;
    let mut v: Vec<u32> = vec![];

    for line in reader.lines() {
        let maybe_c = line?.parse::<u32>();
        if maybe_c.is_ok() {
            r += maybe_c.unwrap();
        } else {
            v.push(r);
            r = 0;
        }
    }

    v.sort();

    let result_2: u32 = v.iter().rev().take(3).sum();

    Ok(println!("{}", result_2))
}
