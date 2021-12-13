use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

#[derive(Clone, Copy)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

static FOLDINGS: [Fold; 12] = [
    Fold::Horizontal(655),
    Fold::Vertical(447),
    Fold::Horizontal(327),
    Fold::Vertical(223),
    Fold::Horizontal(163),
    Fold::Vertical(111),
    Fold::Horizontal(81),
    Fold::Vertical(55),
    Fold::Horizontal(40),
    Fold::Vertical(27),
    Fold::Vertical(13),
    Fold::Vertical(6),
];

fn main() {
    let mut input: HashSet<(usize, usize)> =
        std::io::BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|line| {
                let split: Vec<String> = line.unwrap().split(',').map(|s| s.to_owned()).collect();
                (split[0].parse().unwrap(), split[1].parse().unwrap())
            })
            .collect();

    for fold in FOLDINGS {
        let old_and_new_points: Vec<((usize, usize), (usize, usize))> = input
            .iter()
            .filter(|&&(x, y)| match fold {
                Fold::Horizontal(n) => x > n,
                Fold::Vertical(n) => y > n,
            })
            .map(|p| (*p, symmetry(p, fold)))
            .collect();
        for (old, new) in old_and_new_points {
            input.remove(&old);
            input.insert(new);
        }
    }

    for y in 0..6 {
        for x in 0..50 {
            if input.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn symmetry(point: &(usize, usize), fold: Fold) -> (usize, usize) {
    match fold {
        Fold::Horizontal(n) => (2 * n - point.0, point.1),
        Fold::Vertical(n) => (point.0, 2 * n - point.1),
    }
}
