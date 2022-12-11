fn is_visible(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x == 0 || x == map[0].len() - 1 || y == 0 || y == map.len() - 1 {
        true
    } else {
        let target_height = map[y][x];
        let horizontal_check = map[y][0..x].iter().all(|&height| height < target_height)
            || map[y][x + 1..map[0].len()]
                .iter()
                .all(|&height| height < target_height);
        let vertical_check = (0..y).all(|position| map[position][x] < target_height)
            || (y + 1..map.len()).all(|position| map[position][x] < target_height);
        horizontal_check || vertical_check
    }
}

fn compute_scenic_score(map: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let target_height = map[y][x];
    if x == 0 || x == map[0].len() - 1 || y == 0 || y == map.len() - 1 {
        0
    } else {
        let a = (0..y).rev().take_while(|&yi| map[yi][x] < target_height && yi != 0).count() + 1;
        let mut b = map[y][0..x].iter().rev().take_while(|&&height| height < target_height).count();
        let c = (y+1..map.len()).take_while(|&yi| map[yi][x] < target_height && yi != map.len()-1).count() + 1;
        let mut d = map[y][x+1..map[0].len()].iter().take_while(|&&height| height < target_height).count();
        if d < map[0].len() - x - 1 {
            d += 1;
        }
        a * b * c * d
    }

}

fn main() {
    let map: Vec<Vec<u8>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut result1 = 0usize;
    let mut result2: usize = 0usize;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if is_visible(&map, x, y) {
                result1 += 1;
            }

            let s = compute_scenic_score(&map, x, y);
            if s > result2 {
                result2 = s;
            }
        }
    }

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);

}
