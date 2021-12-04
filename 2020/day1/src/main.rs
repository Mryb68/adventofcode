use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
static SUM_TARGET: u32 = 2020;

fn main() -> Result<(), Error> {
    let mut seen = HashSet::new();
    let input = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    for el in input {
        let diff = SUM_TARGET - el;
        if seen.contains(&diff) {
            println!(
                "multiplication of {} by {} is equal to {}",
                el,
                diff,
                el * diff
            );
            return Ok(());
        } else {
            seen.insert(el);
        }
    }
    Ok(())
}
