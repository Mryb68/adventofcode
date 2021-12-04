use std::io::Error;

fn main() -> Result<(), Error> {
    let input = [17, 1, 3, 16, 19, 0];
    let mut memory: Vec<Option<(usize, Option<usize>)>>;

    for part in 0..=1 {
        let n = if part == 0 { 2020 } else { 30_000_000 };
        memory = vec![None; n];
        let mut i = 1;
        let mut last_spoken = input[0];
        for &inp in input.iter() {
            memory[inp] = Some((i, None));
            i += 1;
            last_spoken = inp;
        }
        while i <= n {
            let (last, for_last) = memory[last_spoken].unwrap();
            if for_last.is_none() {
                last_spoken = 0;
            } else {
                last_spoken = last - for_last.unwrap();
            }
            if let Some((l, _)) = memory[last_spoken] {
                memory[last_spoken] = Some((i, Some(l)));
            } else {
                memory[last_spoken] = Some((i, None));
            }
            i += 1;
        }
        println!("last spoken in part {}: {}", part + 1, last_spoken);
    }

    Ok(())
}
