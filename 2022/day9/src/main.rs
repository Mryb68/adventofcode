use std::collections::HashSet;

fn main() {
    let movement_instructions: Vec<(char, usize)> = include_str!("../input.txt")
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[2..].parse().unwrap()))
        .collect();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut head = (0isize, 0isize);
    let mut tail = (0, 0);
    visited.insert(tail);
    for &(direction, distance) in movement_instructions.iter() {
        for _ in 0..distance {
            match direction {
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                'U' => head.1 += 1,
                _ => head.1 -= 1,
            }
            tail = update_tail(head, tail);
            visited.insert(tail);
        }
    }

    println!("Result 1: {}", visited.len()); //6159
    visited.clear();

    let mut rope: Vec<(isize, isize)> = vec![(0isize, 0isize); 10];
    for (direction, distance) in movement_instructions {
        for _ in 0..distance {
            match direction {
                'L' => rope.first_mut().unwrap().0 -= 1,
                'R' => rope.first_mut().unwrap().0 += 1,
                'U' => rope.first_mut().unwrap().1 += 1,
                _ => rope.first_mut().unwrap().1 -= 1,
            }
            for i in 0..rope.len() - 1 {
                rope[i + 1] = update_tail(rope[i], rope[i + 1]);
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    println!("Result 2: {}", visited.len()); //11844
}

fn update_tail(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    if (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2 {
        tail
    } else if head.0 != tail.0 && head.1 != tail.1 { // diagonal move
        (if tail.0 < head.0 { tail.0 + 1 } else { tail.0 - 1}, if tail.1 < head.1 { tail.1 + 1 } else { tail.1 - 1 })
    } else if head.0 > tail.0 {
        (head.0 - 1, head.1)
    } else if head.0 < tail.0 {
        (head.0 + 1, head.1)
    } else if head.1 > tail.1 {
        (head.0, head.1 - 1)
    } else {
        (head.0, head.1 + 1)
    }
}
