use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/03.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut r: u32 = 0;

    // for maybe_line in reader.lines() {
    //     let line = maybe_line?;
    //     let i = line.len() / 2;
    //     let s1 = &line[..i];
    //     let s2 = &line[i..];
    //     let a = 97;
    //     let v = share_char1 (s1, s2) as u32;
    //     if v >= a {
    //         r += v - 96;
    //     } else {
    //         r += v - 38;
    //     }
    // }
    for (a, b, c) in reader.lines().tuples() {
        let (l1, l2, l3) = (a?, b?, c?);
        let s1 = l1.as_str();
        let s2 = l2.as_str();
        let s3 = l3.as_str();
        let v = share_char2(s1, s2, s3) as u32;
        if v >= 97 {
            r += v - 96;
        } else {
            r += v - 38;
        }
    }

    Ok(println!("{}", r))
}

fn share_char1(a: &str, b: &str) -> char {
    let (s1, s2) = (a, b);

    let set: HashSet<char> = s1.chars().collect();

    for c in s2.chars() {
        if set.contains(&c) {
            return c;
        }
    }

    'ç'
}

fn share_char2(a: &str, b: &str, c: &str) -> char {
    let (s1, s2, s3) = (a, b, c);

    let set: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();

    for c in s3.chars() {
        if set.contains(&c) && set2.contains(&c) {
            return c;
        }
    }

    'ç'
}
