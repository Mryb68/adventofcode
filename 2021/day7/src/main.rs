use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut input = std::io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let crabs = input
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();   

    println!("Minimum fuel for problem 1: {}", compute_min_fuel(&crabs, |d| d));
    println!("Minimum fuel for problem 2: {}", compute_min_fuel(&crabs, |d| d * (d+1) / 2)); // arithmetic progression 1 + 2 + 3 + .. + d = d * (d+1) / 2
}

fn compute_min_fuel(crabs: &[isize], operator: fn(isize) -> isize) -> isize {
    let mut fuel_min: isize = isize::MAX;
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for target in *min..=*max {  
        let mut fuel_used = 0;
        for crab in crabs.iter() {
            let distance = (target - crab).abs();
            fuel_used += operator(distance);
        }
        if fuel_used < fuel_min {
            fuel_min = fuel_used;
        }
    }
    fuel_min
}
