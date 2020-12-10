use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let mut inputs: Vec<u64> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    inputs.push(0);
    inputs.sort();
    let mut jolts = vec![0; 3];
    let mut curr_input = inputs[0];
    for input in &inputs[1..] {
        jolts[(input - curr_input - 1) as usize] += 1;
        curr_input = *input;
    }
    println!("Result 1: {}", jolts[0] * (jolts[2] + 1));

    let mut inputs_split = Vec::new();
    curr_input = inputs[0];
    let mut curr_index = -1;
    for (index, input) in inputs[1..].iter().enumerate() {
        if input - curr_input == 3 {
            inputs_split.push(&inputs[(curr_index + 1) as usize ..= index]);
            curr_index = index as isize;
        }
        curr_input = *input;
    }
    inputs_split.push(&inputs[(curr_index + 1) as usize..]);
    let result: u64 = inputs_split
        .iter()
        .map(|list| count_arrangement(list))
        .product();
    println!("Number of arrangements : {}", result);
    Ok(())
}

fn count_arrangement(inputs: &[u64]) -> u64 {
    let curr = inputs[0];
    if inputs.len() <= 2 {
        1
    } else {
        let third = inputs[2];
        let mut arrangements = count_arrangement(&inputs[1..]);
        if third - curr <= 3 {
            arrangements += count_arrangement(&inputs[2..]);
        }
        if inputs.len() >= 4 && inputs[3] - curr <= 3 {
            arrangements += count_arrangement(&inputs[3..]);
        }
        arrangements
    }
}
