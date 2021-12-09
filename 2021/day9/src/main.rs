use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input: Vec<Vec<u8>> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    // extend map borders to prevent checking the limits.
    let mut input_extended = vec![vec![10u8; input[0].len() + 2]; input.len() + 2];
    for (index, line) in input_extended
        .iter_mut()
        .skip(1)
        .take(input.len())
        .enumerate()
    {
        for (col, val) in line
            .iter_mut()
            .skip(1)
            .take(input[0].len())
            .zip(input[index].iter())
        {
            *col = *val;
        }
    }
    let mut low_points = Vec::new();
    let mut sum: usize = 0;
    for y in 1..=input.len() {
        for x in 1..=input[0].len() {
            let v = input_extended[y][x];
            if v < input_extended[y - 1][x]
                && v < input_extended[y + 1][x]
                && v < input_extended[y][x - 1]
                && v < input_extended[y][x + 1]
            {
                sum += 1 + v as usize;
                low_points.push((y, x))
            }
        }
    }
    println!("Result1: {}", sum);
    let mut marked = HashSet::new();
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|p| compute_basin_size(&input_extended, p, &mut marked))
        .collect();
    basin_sizes.sort_unstable();
    println!(
        "Result2: {}",
        basin_sizes.iter().rev().take(3).product::<usize>()
    );
}

fn compute_basin_size(
    input_extended: &[Vec<u8>],
    current_point: &(usize, usize),
    point_marked: &mut HashSet<(usize, usize)>,
) -> usize {
    let x = current_point.0;
    let y = current_point.1;
    if point_marked.contains(current_point) {
        return 0;
    }
    point_marked.insert(*current_point);
    if input_extended[y][x] >= 9 {
        0
    } else {
        1 + compute_basin_size(input_extended, &(x - 1, y), point_marked)
            + compute_basin_size(input_extended, &(x + 1, y), point_marked)
            + compute_basin_size(input_extended, &(x, y - 1), point_marked)
            + compute_basin_size(input_extended, &(x, y + 1), point_marked)
    }
}
