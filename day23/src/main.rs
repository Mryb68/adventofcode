fn main() {
    let loops_nb = [100, 10_000_000];
    let max = [9, 1_000_000];
    let mut inputs: Vec<u32> = vec![0; 1_000_001].to_vec();
    for part in 0..2 {
        let mut current: usize = 3;
        inputs[3] = 6;
        inputs[6] = 4;
        inputs[4] = 2;
        inputs[2] = 9;
        inputs[9] = 7;
        inputs[7] = 5;
        inputs[5] = 8;
        inputs[8] = 1;
        let mut last: usize = 1;
        if part == 0 {
            inputs[1] = 3;
        } else {
            for i in 10..=1_000_000 {
                inputs[last] = i;
                last = i as usize;
            }
            inputs[last] = 3;
        }

        for _ in 0..loops_nb[part] {
            let mut to_find: usize = current - 1;
            if to_find == 0 {
                to_find = max[part];
            }
            let next1 = inputs[current] as usize;
            let next2 = inputs[next1] as usize;
            let next3 = inputs[next2] as usize;
            let after_next3 = inputs[next3];
            while to_find == next1 || to_find == next2 || to_find == next3 {
                to_find -= 1;
                if to_find == 0 {
                    to_find = max[part];
                }
            }
            let old = inputs[to_find];
            inputs[to_find] = next1 as u32;
            inputs[current] = after_next3;
            inputs[next3] = old;
            current = inputs[current] as usize;
        }

        // Printing results
        let mut curr = inputs[1];
        if part == 0 {
            print!("Part 1 result: ");
            for _ in 0..8 {
                print!("{}", curr);
                curr = inputs[curr as usize];
            }
            println!();
        } else {
            println!(
                "Part 2 result: {}",
                u64::from(curr) * u64::from(inputs[curr as usize])
            );
        }
    }
}
