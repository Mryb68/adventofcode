use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let br = BufReader::new(File::open("input.txt")?);
    let mut input: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();
    for line in br.lines() {
        if let Ok(l) = line {
            if l == "" {
                input.push(current_group.to_owned());
                current_group.clear();
            } else {
                current_group.push(l);
            }
        }
    }
    input.push(current_group);

    let mut total_common = 0;
    let mut total = 0;
    for group in input {
        let mut all_answers = HashSet::new();
        group.iter().for_each(|answers| {
            answers.chars().for_each(|c| {
                all_answers.insert(c);
            })
        });
        total += all_answers.len();
        let count_common = all_answers
            .iter()
            .filter(|&&answer| group.iter().all(|answers| answers.contains(answer)))
            .count();
        total_common += count_common;
    }
    println!(
        "yes total answers: {}, yes common answers {}",
        total, total_common
    );
    Ok(())
}