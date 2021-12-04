use std::collections::HashMap;

static INPUT: &str = "#...#.#.,..#.#.##,..#..#..,.....###,...#.#.#,#.#.##..,#####...,.#.#.##.";
static ACTIVE: u8 = 1;
static INACTIVE: u8 = 0;
static ITERATIONS: isize = 6;

fn main() {
    let input: Vec<Vec<u8>> = INPUT
        .split(',')
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { ACTIVE } else { INACTIVE })
                .collect()
        })
        .collect();

    let level_max = ITERATIONS + 2;
    let size = usize::max(input[0].len(), input.len()) as isize;

    let mut cube: HashMap<(isize, isize, isize, isize), u8> = HashMap::new();
    // Init all possibilities with 0
    for x in -level_max - 1..=level_max + size {
        for y in -level_max - 1..=level_max + size {
            for z in -level_max - 1..=level_max + 1 {
                for w in -level_max - 1..=level_max + 1 {
                    cube.insert((x, y, z, w), 0);
                }
            }
        }
    }
    // Input initialization
    for (l, line) in input.iter().enumerate() {
        for (c, val) in line.iter().enumerate() {
            cube.insert((c as isize, l as isize, 0, 0), *val);
        }
    }
    let mut level = 1;
    for _ in 0..ITERATIONS {
        let c2 = cube.clone();
        for x in -level..level + size {
            for y in -level..level + size {
                for z in -level..=level {
                    for w in -level..=level {
                        let coor = (x, y, z, w);
                        let neightbours_active = adjacents(coor)
                            .iter()
                            .filter(|co| *c2.get(co).unwrap() == ACTIVE)
                            .count();
                        let curr = cube.get_mut(&coor).unwrap();
                        if *curr == ACTIVE {
                            if neightbours_active != 2 && neightbours_active != 3 {
                                *curr = INACTIVE;
                            }
                        } else if neightbours_active == 3 {
                            *curr = ACTIVE;
                        }
                    }
                }
            }
        }
        level += 1;
    }

    println!("{}", cube.values().filter(|&&v| v == ACTIVE).count());
}

fn adjacents(coor: (isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
    let (x, y, z, w) = coor;
    [
        (x - 1, y, z, w),
        (x - 1, y, z - 1, w),
        (x - 1, y, z + 1, w),
        (x - 1, y - 1, z, w),
        (x - 1, y - 1, z - 1, w),
        (x - 1, y - 1, z + 1, w),
        (x - 1, y + 1, z, w),
        (x - 1, y + 1, z - 1, w),
        (x - 1, y + 1, z + 1, w),
        (x - 1, y, z, w - 1),
        (x - 1, y, z - 1, w - 1),
        (x - 1, y, z + 1, w - 1),
        (x - 1, y - 1, z, w - 1),
        (x - 1, y - 1, z - 1, w - 1),
        (x - 1, y - 1, z + 1, w - 1),
        (x - 1, y + 1, z, w - 1),
        (x - 1, y + 1, z - 1, w - 1),
        (x - 1, y + 1, z + 1, w - 1),
        (x - 1, y, z, w + 1),
        (x - 1, y, z - 1, w + 1),
        (x - 1, y, z + 1, w + 1),
        (x - 1, y - 1, z, w + 1),
        (x - 1, y - 1, z - 1, w + 1),
        (x - 1, y - 1, z + 1, w + 1),
        (x - 1, y + 1, z, w + 1),
        (x - 1, y + 1, z - 1, w + 1),
        (x - 1, y + 1, z + 1, w + 1),
        (x, y, z - 1, w),
        (x, y, z + 1, w),
        (x, y - 1, z, w),
        (x, y - 1, z - 1, w),
        (x, y - 1, z + 1, w),
        (x, y + 1, z, w),
        (x, y + 1, z - 1, w),
        (x, y + 1, z + 1, w),
        (x, y, z, w - 1),
        (x, y, z - 1, w - 1),
        (x, y, z + 1, w - 1),
        (x, y - 1, z, w - 1),
        (x, y - 1, z - 1, w - 1),
        (x, y - 1, z + 1, w - 1),
        (x, y + 1, z, w - 1),
        (x, y + 1, z - 1, w - 1),
        (x, y + 1, z + 1, w - 1),
        (x, y, z, w + 1),
        (x, y, z - 1, w + 1),
        (x, y, z + 1, w + 1),
        (x, y - 1, z, w + 1),
        (x, y - 1, z - 1, w + 1),
        (x, y - 1, z + 1, w + 1),
        (x, y + 1, z, w + 1),
        (x, y + 1, z - 1, w + 1),
        (x, y + 1, z + 1, w + 1),
        (x + 1, y, z, w),
        (x + 1, y, z - 1, w),
        (x + 1, y, z + 1, w),
        (x + 1, y - 1, z, w),
        (x + 1, y - 1, z - 1, w),
        (x + 1, y - 1, z + 1, w),
        (x + 1, y + 1, z, w),
        (x + 1, y + 1, z - 1, w),
        (x + 1, y + 1, z + 1, w),
        (x + 1, y, z, w - 1),
        (x + 1, y, z - 1, w - 1),
        (x + 1, y, z + 1, w - 1),
        (x + 1, y - 1, z, w - 1),
        (x + 1, y - 1, z - 1, w - 1),
        (x + 1, y - 1, z + 1, w - 1),
        (x + 1, y + 1, z, w - 1),
        (x + 1, y + 1, z - 1, w - 1),
        (x + 1, y + 1, z + 1, w - 1),
        (x + 1, y, z, w + 1),
        (x + 1, y, z - 1, w + 1),
        (x + 1, y, z + 1, w + 1),
        (x + 1, y - 1, z, w + 1),
        (x + 1, y - 1, z - 1, w + 1),
        (x + 1, y - 1, z + 1, w + 1),
        (x + 1, y + 1, z, w + 1),
        (x + 1, y + 1, z - 1, w + 1),
        (x + 1, y + 1, z + 1, w + 1),
    ]
    .to_vec()
}
