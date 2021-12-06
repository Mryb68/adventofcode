use std::fs::File;
use std::io::BufRead;

static RESET_AGE: usize = 6;

fn main() {
    let mut lines = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .take(1);
    let fishes: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut population = vec![0; 9];
    // initialisation of the population
    for (index, fish_nb) in population.iter_mut().enumerate() {
        *fish_nb = fishes.iter().filter(|&&age| age == index as u8).count();
    }
    // compute result
    println!(
        "Result1: {}",
        compute_population(80, &mut population.clone(), 0)
    );
    println!("Result2: {}", compute_population(256, &mut population, 0));
}

fn compute_population(final_day: usize, population: &mut [usize], current_day: usize) -> usize {
    if current_day > final_day {
        panic!("current day cannot be greater than final_day");
    }
    if current_day == final_day {
        population.iter().sum()
    } else {
        let nb_birth = population[0];
        for i in 0..population.len() - 1 {
            population[i] = population[i + 1];
        }
        population[RESET_AGE] += nb_birth;
        population[population.len() - 1] = nb_birth;
        compute_population(final_day, population, current_day + 1)
    }
}
