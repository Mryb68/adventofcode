use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input: Vec<(Vec<char>, HashSet<char>)> =
        std::io::BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .flatten()
            .map(|line| {
                let len = line.len() / 2;
                (
                    line[0..len].chars().collect(),
                    line[len..].chars().collect(),
                )
            })
            .collect();

    let r1: usize = input
        .iter()
        .map(|(items, set)| {
            for item in items.iter() {
                if set.contains(item) {
                    if item > &'Z' {
                        return *item as usize - 96usize;
                    } else {
                        return *item as usize - 38usize;
                    }
                }
            }
            0usize
        })
        .sum();

    println!("result1 : {}", r1);

    let result2: usize =
        std::io::BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .flatten()
            .collect::<Vec<String>>()
            .chunks(3)
            .map(|chunk| {
                let set1: HashSet<char> = chunk[0].chars().collect();
                let set2: HashSet<char> = chunk[1].chars().collect();
                for item in chunk[2].chars() {
                    if set1.contains(&item) && set2.contains(&item) {
                        if item > 'Z' {
                            return item as usize - 96usize;
                        } else {
                            return item as usize - 38usize;
                        }
                    }
                }
                0usize
    
            })
            .sum();

    println!("result2 : {}", result2);
}
