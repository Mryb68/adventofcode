use std::fs::File;
use std::io::BufRead;

fn main() {
    let input: Vec<((usize, usize), (usize, usize))> =
        std::io::BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .flatten()
            .map(|line| {
                let mut pairs = line.split(',');
                let mut s1 = pairs.next().unwrap().split('-');
                let mut s2 = pairs.next().unwrap().split('-');
                (
                    (
                        s1.next().unwrap().parse().unwrap(),
                        s1.next().unwrap().parse().unwrap(),
                    ),
                    (
                        s2.next().unwrap().parse().unwrap(),
                        s2.next().unwrap().parse().unwrap(),
                    ),
                )
            })
            .collect();

    let r1: usize = input
        .iter()
        .filter(|&((a1, a2), (b1, b2))| (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2))
        .count();
    println!("Result1: {}", r1);

    let r2: usize = input
        .iter()
        .filter(|&((a1, a2), (b1, b2))| (a2 >= b1 && a1 <= b1) || (a1 >= b1 && a1 <= b2))
        .count();
    println!("Result2: {}", r2);
}
