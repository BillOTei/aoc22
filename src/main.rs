use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/04.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut r: u32 = 0;
    let re = Regex::new(r"\d+").unwrap();

    for maybe_line in reader.lines() {
        let line = maybe_line?;
        let matches = re.find_iter(&line);
        for (m1, m2, m3, m4) in matches.tuples() {
            let (l1, l2, l3, l4) = (m1.as_str(), m2.as_str(), m3.as_str(), m4.as_str());
            let (s1, s2, s3, s4): (u32, u32, u32, u32) = (
                l1.parse().unwrap(),
                l2.parse().unwrap(),
                l3.parse().unwrap(),
                l4.parse().unwrap(),
            );
            if overlap(s1, s2, s3, s4) {
                r += 1;
            }
        }
    }

    Ok(println!("{}", r))
}

fn included(s1: u32, s2: u32, s3: u32, s4: u32) -> bool {
    s1 <= s3 && s2 >= s4 || s1 >= s3 && s2 <= s4
}

fn overlap(s1: u32, s2: u32, s3: u32, s4: u32) -> bool {
    let v1 = vec![s1, s3];
    let v2 = vec![s2, s4];
    let i1 = v1.iter().max();
    let i2 = v2.iter().min();

    i1.unwrap() <= i2.unwrap()
}
