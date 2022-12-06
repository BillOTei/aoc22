use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Result;
use std::str::Chars;

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/06.txt");
    let str_input = fs::read_to_string(path)?;
    let chars = str_input.chars();

    Ok(println!("{:?}", get_subroutine(chars, &str_input)))
}

fn get_subroutine(chars: Chars, str: &str) -> usize {
    let it = chars.enumerate();
    let l = str.len();
    let window_size = 14;
    for (i, _) in it {
        if i + window_size <= l {
            let window = &str[i..i + window_size];
            let mut a = window.chars().collect::<Vec<char>>();
            let mut uniques = HashSet::new();
            a.retain(|e| uniques.insert(e.clone()));

            if a.len() == window.len() {
                return i + window_size;
            }
        }
    }

    0
}
