use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/02.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut r: u32 = 0;

    for maybe_line in reader.lines() {
        let line = maybe_line?;
        let opp = &line[0..1];
        let own = &line[2..];

        r += play2(own, opp);
    }

    Ok(println!("{}", r))
}

fn play1(own: &str, opp: &str) -> u32 {
    match opp {
        "A" => {
            if own == "X" {
                3 + 1
            } else if own == "Y" {
                6 + 2
            } else {
                3
            }
        }
        "B" => {
            if own == "X" {
                1
            } else if own == "Y" {
                3 + 2
            } else {
                6 + 3
            }
        }
        "C" => {
            if own == "X" {
                6 + 1
            } else if own == "Y" {
                2
            } else {
                3 + 3
            }
        }
        _ => 0,
    }
}

fn play2(own: &str, opp: &str) -> u32 {
    match opp {
        "A" => {
            if own == "X" {
                0 + 3
            } else if own == "Y" {
                3 + 1
            } else {
                6 + 2
            }
        }
        "B" => {
            if own == "X" {
                0 + 1
            } else if own == "Y" {
                3 + 2
            } else {
                6 + 3
            }
        }
        "C" => {
            if own == "X" {
                0 + 2
            } else if own == "Y" {
                3 + 3
            } else {
                6 + 1
            }
        }
        _ => 0,
    }
}
