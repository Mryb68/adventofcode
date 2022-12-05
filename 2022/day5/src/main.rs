use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let mut stack: Vec<VecDeque<char>> = vec![
        VecDeque::from(['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T']),
        VecDeque::from(['Q', 'R', 'B']),
        VecDeque::from(['B', 'Z', 'T', 'Q', 'P', 'M', 'S']),
        VecDeque::from(['D', 'V', 'F', 'R', 'Q', 'H']),
        VecDeque::from(['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P']),
        VecDeque::from(['W', 'R', 'T', 'Z']),
        VecDeque::from(['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J']),
        VecDeque::from(['R', 'N', 'F', 'H', 'W']),
        VecDeque::from(['J', 'Z', 'T', 'Q', 'P', 'R', 'B']),
    ];

    let mut stack2 = stack.clone();

    let instructions: Vec<(usize, usize, usize)> =
        std::io::BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .flatten()
            .map(|line| {
                let mut split = line.split(' ');
                (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

    // CrateMover 9000 Algo
    for &(quantity, source, target) in instructions.iter() {
        for _ in 0..quantity {
            let c = stack[source - 1].pop_back().unwrap();
            stack[target - 1].push_back(c);
        }
    }

    // CrateMover 9001 Algo
    let mut accumulator = VecDeque::new();
    for &(quantity, source, target) in instructions.iter() {
        for _ in 0..quantity {
            accumulator.push_back(stack2[source - 1].pop_back().unwrap());
        }
        while !accumulator.is_empty() {
            stack2[target - 1].push_back(accumulator.pop_back().unwrap());
        }
    }

    let print_result = |s: Vec<VecDeque<char>>| {
        for vd in s.iter() {
            print!("{}", vd.back().unwrap());
        }
        println!();
    };
    print!("Result 1: ");
    print_result(stack);
    print!("Result 2: ");
    print_result(stack2);
}
