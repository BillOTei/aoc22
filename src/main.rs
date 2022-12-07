use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/07.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let str_input = fs::read_to_string(path)?;

    let mut sum: u32 = 0;
    let threshold: u32 = 100000;
    let mut current_dir = "".to_owned();
    let mut dir_sizes: BTreeMap<String, u32> = BTreeMap::new();

    for maybe_line in reader.lines() {
        let line = maybe_line?;
        if &line[0..1] == "$" {
            if &line[2..4] == "ls" {
            } else {
                let folder = &line[5..];
                if folder != ".." {
                    current_dir.push_str("/");
                    current_dir.push_str(folder);
                } else {
                    let idx = current_dir.rfind("/").unwrap();
                    current_dir.truncate(idx);
                }
            }
        } else {
            let maybe_file_size = line.split(" ").next().unwrap();
            if maybe_file_size != "dir" {
                let file_size: u32 = maybe_file_size.parse().unwrap();
                *dir_sizes.entry(current_dir.clone()).or_insert(0) += file_size;
            }
        }
    }

    let mut directories = HashMap::new();
    for (path, size) in dir_sizes.into_iter() {
        let path_list = path.split("/").map(|s| s.to_owned()).collect_vec();
        for to in 1..=path_list.len() {
            match directories.get_mut(&path_list[..to]) {
                Some(total) => {
                    *total += size;
                }
                None => {
                    directories.insert(path_list[..to].to_vec(), size);
                }
            }
        }
    }

    directories.retain(|_, v| *v < threshold);
    for v in directories.values() {
        sum += v;
    }

    Ok(println!("{:?}", sum))
}
