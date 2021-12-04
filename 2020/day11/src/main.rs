use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
    Empty,
    Occupied,
    Floor,
    Error
}

fn main() -> Result<(), Error> {
    let mut inputs : Vec<Vec<State>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.chars().map(|c| match c {
          'L' => State::Empty,
          '#' => State::Occupied,
          '.' => State::Floor,
          _ => State::Error
        }).collect())
        .collect();

    let mut inputs2 : Vec<Vec<State>>= Vec::new();


    for input in inputs.iter() {
        inputs2.push(input.to_vec());
    } 

    loop {
        let mut has_changed = false;
        for (y, line) in inputs.iter_mut().enumerate() {
            for (x, val) in line.iter_mut().enumerate() {
                //let seets = adjacent_seats(&inputs2[..], y, x);
                // if *val == State::Empty && !seets.iter().any(|s| *s == State::Occupied) {
                //     *val = State::Occupied;
                //     has_changed = true;
                // } else if *val == State::Occupied && seets.iter().filter(|&&s| s == State::Occupied).count() >= 4 {
                //     *val = State::Empty;
                //     has_changed = true;
                // }
                let occupied_nb = count_occupied(&inputs2, y, x);
                if *val == State::Empty && occupied_nb == 0 {
                    *val = State::Occupied;
                    //println!("empty seet -> occupied");
                    has_changed = true;
                } else if *val == State::Occupied &&  occupied_nb >= 5 {
                    *val = State::Empty;
                    //println!("occupied seet -> empty");
                    has_changed = true;
                }
            }
        } 
        if !has_changed {
            break;
        }
        inputs2.clear();
        for input in inputs.iter() {
            inputs2.push(input.to_vec());
        } 

        // println!("\n");
        // inputs.iter().map(|line| line.iter().map(|&state| {
        //     if state == State::Occupied { '#' } else if state == State::Empty {'L'} else {'.'}
        // }).collect::<String>()).for_each(|line| {
        //     println!("{}", line);
        // });
    }

    let occupied: usize = inputs.iter().map(|line| line.iter().filter(|&&s| s == State::Occupied).count()).sum();
    println!("seeats occupied: {}", occupied);
    
    Ok(())
}

fn adjacent_seats(input: &[Vec<State>], line: usize, col: usize) -> Vec<State> {
    let mut adjacent = Vec::new();
    let w = input[0].len();
    let h = input.len();
    if col < w - 1 {
        adjacent.push(input[line][col+1]);
        if line < h-1 {
            adjacent.push(input[line+1][col+1]);
        } 
        if line > 0 {
            adjacent.push(input[line-1][col+1]);
        }
    }
    if col > 0 {
        adjacent.push(input[line][col-1]);
        if line < h - 1 {
            adjacent.push(input[line+1][col-1]);
        } 
        if line > 0 {
            adjacent.push(input[line-1][col-1]);
        }
    }
    if line < h - 1 {
        adjacent.push(input[line+1][col]);
    }
    if line > 0 {
        adjacent.push(input[line-1][col]);
    } 
    adjacent
}

fn check_seet_occupied(input: &[Vec<State>], line: usize, col: usize, occ_count: &mut usize) -> bool {
    if input[line][col] == State::Occupied {
        *occ_count += 1;
        true
    } else if input[line][col] == State::Empty {
        true
    } else {
        false
    }
}

fn count_occupied(input: &[Vec<State>], line: usize, col: usize) -> usize {
    let mut nb_occ = 0;
    let w = input[0].len();
    let h = input.len();
    let mut i = 1;
    while col + i < w {
        if check_seet_occupied(input, line, col + i, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while col as isize - i as isize >= 0 {
        if check_seet_occupied(input, line, col - i, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while line + i < h {
        if check_seet_occupied(input, line + i, col, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while line as isize - i as isize >= 0 {
        if check_seet_occupied(input, line - i, col, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    //
    i = 1;
    while col + i < w && line + i < h  {
        if check_seet_occupied(input, line + i, col + i, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while col as isize - i as isize >= 0 && line as isize - i as isize >= 0{
        if check_seet_occupied(input, line - i, col - i, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while line + i < h && col as isize - i as isize >= 0 {
        if check_seet_occupied(input, line + i, col - i, &mut nb_occ) {
            break;
        }
        i += 1;
    }
    i = 1;
    while line as isize - i as isize >= 0 && col + i < w {
        if check_seet_occupied(input, line - i, col + i, &mut nb_occ) {
            break;
        }
        i += 1;
    }

    nb_occ
} 