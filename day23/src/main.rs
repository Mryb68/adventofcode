use std::collections::HashMap;
fn main() {
    let loops_nb = [100, 10_000_000];
    let max = [9, 1_000_000];
    let mut inputs: HashMap<u32, u32> = HashMap::with_capacity(1_000_000);
    for part in 0..2 {
        let mut current = 3;
        inputs.clear();
        inputs.insert(3, 6);
        inputs.insert(6, 4);
        inputs.insert(4, 2);
        inputs.insert(2, 9);
        inputs.insert(9, 7);
        inputs.insert(7, 5);
        inputs.insert(5, 8);
        inputs.insert(8, 1);
        let mut last = 1;
        if part == 0 {
            inputs.insert(last, 3);
        } else {
            for i in 10..=1_000_000 {
                inputs.insert(last, i);
                last = i;
            }
            inputs.insert(last, 3);
        }

        for _ in 0..loops_nb[part] {
            let mut to_find = current - 1;
            if to_find == 0 {
                to_find = max[part];
            }
            let next1 = inputs[&current];
            let next2 = inputs[&next1];
            let next3 = inputs[&next2];
            let after_next3 = inputs[&next3];
            while to_find == next1 || to_find == next2 || to_find == next3 {
                to_find -= 1;
                if to_find == 0 {
                    to_find = max[part];
                }
            }
            let insert = inputs.get_mut(&to_find).unwrap();
            let old = *insert;
            *insert = next1;
            let curr = inputs.get_mut(&current).unwrap();
            *curr = after_next3;
            let new_after3 = inputs.get_mut(&next3).unwrap();
            *new_after3 = old;
            current = inputs[&current];
        }

        // Printing results
        let mut curr = inputs[&1];
        if part == 0 {
            print!("Part 1 result: ");
            for _ in 0..8 {
                print!("{}", curr);
                curr = inputs[&curr];
            }
            println!();
        } else {
            println!(
                "Part 2 result: {}",
                u64::from(curr) * u64::from(inputs[&curr])
            );
        }
    }
}
