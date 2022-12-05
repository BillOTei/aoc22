use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Result;

pub(crate) fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("src/puzzles/05.txt");
    let str_input = fs::read_to_string(path)?;

    let mut cranes: Vec<String> = vec![
        String::from("FLMW"),
        String::from("FMVZB"),
        String::from("QLSRVH"),
        String::from("JTMPQVSF"),
        String::from("WSL"),
        String::from("WJRMPVF"),
        String::from("FRNPCQJ"),
        String::from("BRWZSPHV"),
        String::from("WZHGCJMB"),
    ];

    let re = &Regex::new(r"\d+").unwrap();
    let splits = str_input.split("\n\n");
    for (_, moves) in splits.tuples() {
        let matches = re.find_iter(moves);
        for (m1, m2, m3) in matches.tuples() {
            let amount: usize = usize::from_str_radix(m1.as_str(), 10).unwrap();
            let start_idx: usize = usize::from_str_radix(m2.as_str(), 10).unwrap() - 1;
            let end_idx: usize = usize::from_str_radix(m3.as_str(), 10).unwrap() - 1;

            let remain: String = cranes[start_idx][amount..].to_string();
            let mut to_update: String = cranes[start_idx][..amount]
                .to_string()
                .chars()
                .rev()
                .collect();
            let to_move = cranes[end_idx].to_string();
            to_update.push_str(&to_move);
            cranes[start_idx] = remain;
            cranes[end_idx] = to_update;
        }
    }

    Ok(println!("{:?}", cranes))
}
