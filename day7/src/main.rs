use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let mut book: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    BufReader::new(File::open("input.txt")?)
        .lines()
        .for_each(|line| {
            if let Ok(l) = line {
                let container = l.split(',').next().unwrap();
                book.insert(container.to_owned(), Vec::new());
                l.split(',')
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .chunks(2)
                    .for_each(|w| {
                        book.get_mut(container)
                            .unwrap()
                            .push((w[1].to_owned(), w[0].parse::<u32>().unwrap()));
                    });
            }
        });
    let my_bag = "shiny gold";
    let containers_nb = book
        .keys()
        .filter(|bag| contains_bag(&book, bag, my_bag))
        .count();
    let bags_in_my_bag = count_bags(&book, my_bag) - 1;
    println!("number of bags containing shiny gold: {}", containers_nb);
    println!("number of bags in shiny gold: {}", bags_in_my_bag);
    Ok(())
}

fn contains_bag(book: &HashMap<String, Vec<(String, u32)>>, key: &str, bag_to_find: &str) -> bool {
    if let Some(bags) = book.get(key) {
        bags.iter().fold(false, |acc, bag| {
            acc || bag.0 == bag_to_find || contains_bag(book, &bag.0, bag_to_find)
        })
    } else {
        false
    }
}

fn count_bags(book: &HashMap<String, Vec<(String, u32)>>, key: &str) -> u32 {
    book.get(key)
        .unwrap()
        .iter()
        .fold(1, |acc, bag| acc + bag.1 * count_bags(book, &bag.0))
}
