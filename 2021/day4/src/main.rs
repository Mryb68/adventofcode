use std::fs::File;
use std::io::BufRead;

static DRAW: &[usize] = &[
    73, 42, 95, 35, 13, 40, 99, 92, 33, 30, 83, 1, 36, 93, 59, 90, 55, 25, 77, 44, 37, 62, 41, 47,
    80, 23, 51, 61, 21, 20, 76, 8, 71, 34, 58, 5, 52, 22, 39, 57, 17, 2, 26, 0, 10, 72, 19, 3, 64,
    65, 82, 46, 31, 63, 91, 24, 18, 12, 9, 79, 50, 98, 69, 4, 78, 54, 43, 68, 87, 7, 67, 48, 28,
    89, 94, 53, 85, 81, 49, 88, 6, 96, 29, 56, 97, 66, 38, 16, 32, 70, 74, 27, 84, 86, 45, 75, 60,
    15, 14, 11,
];

struct Board {
    data: Vec<usize>,
    mask: [bool; 25],
    has_won: bool,
}

impl Board {
    pub fn from_string(input: &[String]) -> Self {
        let mut data = Vec::with_capacity(25);
        for line in input {
            line.split_whitespace()
                .for_each(|number| data.push(number.parse().unwrap()));
        }
        Board {
            data,
            mask: [false; 25],
            has_won: false,
        }
    }

    fn check_col(&self, row_index: usize) -> bool {
        let mut valid = true;
        for i in 0..5 {
            valid &= self.mask[5 * i + row_index];
        }
        valid
    }

    fn check_row(&self, line_index: usize) -> bool {
        self.mask[line_index * 5..(line_index + 1) * 5]
            .iter()
            .all(|&validated| validated)
    }

    pub fn process(&mut self, number: usize) {
        if self.has_won {
            return;
        }
        let index_validated = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, &grid_number)| grid_number == number)
            .map(|(i, _)| i);
        for index in index_validated {
            self.mask[index] = true;
        }
        self.has_won =
            (0..5).any(|row| self.check_row(row)) || (0..5).any(|col| self.check_col(col));
        if self.has_won {
            println!("Board wins with the result {}", self.compute_result(number));
        }
    }

    pub fn compute_result(&self, draw: usize) -> usize {
        self.mask
            .iter()
            .zip(self.data.iter())
            .filter(|(&valid, _)| !valid)
            .map(|(_, n)| n)
            .sum::<usize>()
            * draw
    }
}

fn main() {
    let input: Vec<String> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let nb_boards = input.len() / 5;
    let mut boards = Vec::with_capacity(nb_boards);
    for i in 0..nb_boards {
        boards.push(Board::from_string(&input[i * 5..(i + 1) * 5]));
    }

    for d in DRAW {
        for board in boards.iter_mut() {
            board.process(*d);
        }
    }
}
