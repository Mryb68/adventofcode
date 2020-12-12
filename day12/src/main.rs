use std::io::Error;

static COS_ROT: [isize; 4] = [1, 0, -1, 0];
static SIN_ROT: [isize; 4] = [0, 1, 0, -1];

fn main() -> Result<(), Error> {
    let inputs: Vec<(char, isize)> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect();

    let mut current_coor = [0, 0];
    let mut current_dir = [1, 0];

    for dir in inputs.iter() {
        match dir {
            ('F', n) => {
                current_coor = [
                    current_coor[0] + n * current_dir[0],
                    current_coor[1] + n * current_dir[1],
                ]
            }
            ('N', n) => current_coor[1] += n,
            ('S', n) => current_coor[1] -= n,
            ('E', n) => current_coor[0] += n,
            ('W', n) => current_coor[0] -= n,
            ('L', n) => {
                turn(*n as usize, &mut current_dir, false);
            }
            ('R', n) => {
                turn(*n as usize, &mut current_dir, true);
            }
            _ => {}
        }
    }
    println!(
        "coordinates 1: {}",
        isize::abs(current_coor[0]) + isize::abs(current_coor[1])
    );

    let mut waypoint = [10, 1];
    current_coor = [0, 0];
    for dir in inputs.iter() {
        match dir {
            ('F', n) => {
                current_coor = [
                    current_coor[0] + waypoint[0] * n,
                    current_coor[1] + waypoint[1] * n,
                ]
            }
            ('N', n) => waypoint[1] += n,
            ('S', n) => waypoint[1] -= n,
            ('E', n) => waypoint[0] += n,
            ('W', n) => waypoint[0] -= n,
            ('L', n) => {
                turn(*n as usize, &mut waypoint, false);
            }
            ('R', n) => {
                turn(*n as usize, &mut waypoint, true);
            }
            _ => {}
        }
    }
    println!(
        "coordinates 2: {}",
        isize::abs(current_coor[0]) + isize::abs(current_coor[1])
    );
    Ok(())
}

fn turn(angle: usize, dir: &mut [isize; 2], right: bool) {
    let a = (angle / 90) % 4;
    let factor = if right { -1 } else { 1 };
    *dir = [
        COS_ROT[a] * dir[0] - factor * SIN_ROT[a] * dir[1],
        COS_ROT[a] * dir[1] + factor * SIN_ROT[a] * dir[0],
    ];
}
