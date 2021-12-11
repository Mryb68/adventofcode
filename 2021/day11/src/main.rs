use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut input: Vec<Vec<i8>> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut flashed = HashSet::new();
    let mut count = 0;
    let mut i = 1;
    loop {
        //1/ Increment levels
        increment_levels(&mut input);
        //2/ Flash
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] > 9 {
                    flash(&mut input, y, x, &mut flashed);
                }
            }
        }
        //3/ Reset flashed + update counter
        count += flashed.len();
        if flashed.len() == input.len() * input[0].len() {
            println!("Result 2: All octupus flashed at step {}", i);
            return;
        }
        for &(y, x) in flashed.iter() {
            input[y][x] = 0;
        }
        flashed.clear();
        if i == 100 {
            println!("Result 1: Total number of flash at step 100 is {}", count);
        }
        i += 1;
    }
}

fn increment_levels(input: &mut [Vec<i8>]) {
    for l in input.iter_mut() {
        for c in l.iter_mut() {
            *c += 1;
        }
    }
}

fn flash(
    input: &mut [Vec<i8>],
    y: usize,
    x: usize,
    flashed: &mut HashSet<(usize, usize)>,
) {
    if y >= input.len() || x >= input[0].len() {
        return;
    }
    if flashed.contains(&(y, x)) {
        return;
    }
    input[y][x] += 1;
    if input[y][x] > 9 {
        flashed.insert((y, x));
        flash(input, y, x + 1, flashed);
        flash(input, y + 1, x + 1, flashed);
        flash(input, y + 1, x, flashed);
        if y > 0 {
            flash(input, y - 1, x, flashed);
            flash(input, y - 1, x + 1, flashed);
        }
        if x > 0 {
            flash(input, y, x - 1, flashed);
            flash(input, y + 1, x - 1, flashed);
        }
        if y > 0 && x > 0 {
            flash(input, y - 1, x - 1, flashed);
        }
    }
}
