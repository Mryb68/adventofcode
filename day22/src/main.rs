use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let p1: Vec<usize> = vec![
        42, 29, 12, 40, 47, 26, 11, 39, 41, 13, 8, 50, 44, 33, 5, 27, 10, 25, 17, 1, 28, 22, 6, 32,
        35,
    ];
    let p2: Vec<usize> = vec![
        19, 34, 38, 21, 43, 14, 23, 46, 16, 3, 36, 31, 37, 45, 30, 15, 49, 48, 24, 9, 2, 18, 4, 7,
        20,
    ];

    // let p1: Vec<usize> = vec![9, 2, 6, 3, 1];
    // let p2: Vec<usize> = vec![5, 8, 4, 7, 10];
    let mut vd1 = VecDeque::new();
    let mut vd2 = VecDeque::new();
    // Part 1
    for p in p1.iter() {
        vd1.push_back(*p);
    }
    for p in p2.iter() {
        vd2.push_back(*p);
    }
    let mut n = vd1.len();
    while n > 0 {
        let c1 = vd1.pop_front().unwrap();
        let c2 = vd2.pop_front().unwrap();
        if c1 > c2 {
            vd1.push_back(c1);
            vd1.push_back(c2);
        } else {
            vd2.push_back(c2);
            vd2.push_back(c1);
        }
        n = usize::min(vd1.len(), vd2.len());
    }

    let result: usize = vd1.iter().rev().enumerate().map(|(i, c)| c * (i + 1)).sum();
    println!("Result Part 1: {}", result);
    // Answer: 32495

    // Part 2
    vd1.clear();
    vd2.clear();
    for p in p1.iter() {
        vd1.push_back(*p);
    }
    for p in p2.iter() {
        vd2.push_back(*p);
    }
    let result: usize = if battle(&mut vd1, &mut vd2, 1) {
        vd1.iter().rev().enumerate().map(|(i, c)| c * (i + 1)).sum()
    } else {
        vd2.iter().rev().enumerate().map(|(i, c)| c * (i + 1)).sum()
    };
    println!("Result Part 2: {}", result);
    // Answer: 32665
}

fn get_config(v1: &VecDeque<usize>, v2: &VecDeque<usize>) -> (String, String) {
    let s1 = v1
        .iter()
        .map(|n| {
            let mut s = n.to_string();
            s.push_str(",");
            s
        })
        .collect();
    let s2 = v2
        .iter()
        .map(|n| {
            let mut s = n.to_string();
            s.push_str(",");
            s
        })
        .collect();
    (s1, s2)
}

// return True means P1 wins
fn battle(v1: &mut VecDeque<usize>, v2: &mut VecDeque<usize>, game: usize) -> bool {
    let mut loop_checker = HashSet::new();
    loop {
        if v1.is_empty() {
            return false;
        }
        if v2.is_empty() {
            return true;
        }
        let c = get_config(&v1, &v2);
        if loop_checker.contains(&c) {
            return true;
        }
        loop_checker.insert(c);
        let c1 = v1.pop_front().unwrap();
        let c2 = v2.pop_front().unwrap();

        if c1 <= v1.len() && c2 <= v2.len() {
            let mut v1c = VecDeque::new();
            let mut v2c = VecDeque::new();
            for v in v1.iter().take(c1) {
                v1c.push_back(*v);
            }
            for v in v2.iter().take(c2) {
                v2c.push_back(*v);
            }
            if battle(&mut v1c, &mut v2c, game + 1) {
                v1.push_back(c1);
                v1.push_back(c2);
            } else {
                v2.push_back(c2);
                v2.push_back(c1);
            }
            continue;
        }

        if c1 > c2 {
            v1.push_back(c1);
            v1.push_back(c2);
        } else {
            v2.push_back(c2);
            v2.push_back(c1);
        }
    }
}
