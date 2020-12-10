use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let inputs : Vec<u64> = BufReader::new(File::open("input.txt")?)
        .lines().map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut index = 25;
    let mut invalid = 0;
    for to_check in &inputs[index..] {
        if !inputs[index-25..index].iter().combinations(2).any(|pair| pair[0] + pair[1] == *to_check) {
            println!("{} does not follow the rule", to_check);
            invalid = *to_check;
            break;
        }
        index += 1
    }
    for i in 0..inputs.len() {
        let mut acc = 0;
        for (j, el) in inputs[i..].iter().enumerate() {
            acc += el;
            if acc == invalid {
                println!("result: {}", inputs[i..=j+i].iter().min().unwrap() + inputs[i..=j+i].iter().max().unwrap());
                return Ok(())
            }
        }        
    }
    Ok(())
}
