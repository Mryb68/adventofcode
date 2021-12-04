use std::collections::HashSet;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input: Vec<Vec<(isize, isize)>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| match s {
                    "e" => (2, 0),
                    "w" => (-2, 0),
                    "se" => (1, -1),
                    "ne" => (1, 1),
                    "sw" => (-1, -1),
                    "nw" => (-1, 1),
                    _ => (0, 0),
                })
                .collect()
        })
        .collect();

    let mut black_tiles = HashSet::new();

    for v in input.iter() {
        let mut dest = [0, 0];
        for (dx, dy) in v.iter() {
            dest[0] += dx;
            dest[1] += dy;
        }
        if black_tiles.contains(&dest) {
            black_tiles.remove(&dest);
        } else {
            black_tiles.insert(dest);
        }
    }
    println!("Result of part1: {}", black_tiles.len());

    let mut tiles_copy = black_tiles.clone();
    for _ in 0..100 {
        let mut y = -70;
        while y < 70 {
            let mut x = -110 + y % 2;
            while x < 110 {
                let neighbours = get_adj(x, y);
                let mut count_black = 0;
                for (nx, ny) in neighbours {
                    if tiles_copy.contains(&[nx, ny]) {
                        count_black += 1;
                    }
                }
                if tiles_copy.contains(&[x, y]) && (count_black == 0 || count_black > 2) {
                    black_tiles.remove(&[x, y]);
                }
                if !tiles_copy.contains(&[x, y]) && count_black == 2 {
                    black_tiles.insert([x, y]);
                }
                x += 2;
            }
            y += 1;
        }
        tiles_copy = black_tiles.clone();
    }

    println!("Result of part2: {}", black_tiles.len());
    //3787

    Ok(())
}

fn get_adj(x: isize, y: isize) -> Vec<(isize, isize)> {
    [
        (x + 2, y),
        (x + 1, y - 1),
        (x - 1, y - 1),
        (x - 2, y),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ]
    .to_vec()
}
