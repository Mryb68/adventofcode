use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

struct Line {
    pub p1: [isize; 2],
    pub p2: [isize; 2],
}

struct Map {
    pub vents: HashMap<[isize; 2], isize>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            vents: HashMap::new(),
        }
    }

    fn increment(&mut self, coor: &[isize; 2]) {
        if self.vents.contains_key(coor) {
            *self.vents.get_mut(coor).unwrap() += 1;
        } else {
            self.vents.insert(*coor, 1);
        }
    }

    pub fn add_vent(&mut self, line: &Line) {
        let mut sign = [line.p2[0] - line.p1[0], line.p2[1] - line.p1[1]];
        let n = std::cmp::max(isize::abs(sign[0]), isize::abs(sign[1]));
        if sign[0] != 0 {
            sign[0] /= isize::abs(sign[0]);
        }
        if sign[1] != 0 {
            sign[1] /= isize::abs(sign[1]);
        }
        for i in 0..=n {
            self.increment(&[line.p1[0] + i * sign[0], line.p1[1] + i * sign[1]]);
        }
    }

    pub fn count_overlapped_coor(&self) -> usize {
        self.vents.values().filter(|&&v| v > 1).count()
    }
}

impl Line {
    pub fn from_string(input: &str) -> Self {
        let coordinates: Vec<&str> = input.split(" -> ").collect();
        let p1_s: Vec<&str> = coordinates[0].split(',').collect();
        let p2_s: Vec<&str> = coordinates[1].split(',').collect();
        Line {
            p1: [p1_s[0].parse().unwrap(), p1_s[1].parse().unwrap()],
            p2: [p2_s[0].parse().unwrap(), p2_s[1].parse().unwrap()],
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1[1] == self.p2[1]
    }
    pub fn is_vertical(&self) -> bool {
        self.p1[0] == self.p2[0]
    }
}

fn main() {
    let lines: Vec<Line> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| Line::from_string(&line.unwrap()))
        .collect();

    let mut map = Map::new();
    for line in lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
    {
        map.add_vent(line);
    }
    println!("Result 1: {}", map.count_overlapped_coor());
    for line in lines
        .iter()
        .filter(|line| !line.is_horizontal() && !line.is_vertical())
    {
        map.add_vent(line);
    }
    println!("Result 2: {}", map.count_overlapped_coor());
}
