use std::io::Error;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let mut inst = Vec::new();
    let mut inputs: Vec<(String, Vec<(u64, u64)>)> = Vec::new();
    let mut mask: String = String::new();

    std::fs::read_to_string("input.txt")?
        .lines()
        .for_each(|l| {
            let mut numbers = l.split(' ');
            let n1 = numbers.next();
            let n2 = numbers.next();
            if n2.is_none() {
                inputs.push((mask.to_owned(), inst.to_owned()));
                inst.clear();
                mask = n1.unwrap().to_owned();
            } else {
                inst.push((n1.unwrap().parse().unwrap(), n2.unwrap().parse().unwrap()));
            }
        });
    inputs = inputs[1..].to_owned();
    inputs.push((mask, inst));

    let mut memory = HashMap::new();
    for (mask, instrs) in &inputs {
        for instr in instrs.iter() {
            let mut number = instr.1;
            apply_mask(&mask, &mut number);
            memory.insert(instr.0, number);
        }
    }

    println!("Result 1: {}", memory.values().sum::<u64>());

    // PART 2
    memory.clear();
    for group in inputs {
        let mask : String = group.0.to_owned();
        for instr in group.1.iter() {
            let number = instr.1;
            let addr = instr.0;
            let addr_bits : Vec<u8> = (0..36).into_iter().map(|i| get_bit(addr, 35-i)).collect();
            let mut new_mask = String::new();
            for (c, b) in mask.chars().zip(addr_bits.iter()) {
                if c == '0' {
                    new_mask.push(if *b == 1 {'1'} else {'0'});
                } else if c == '1' {
                    new_mask.push('1');
                } else {
                    new_mask.push('X');
                }
            }
            let masks = build_masks(&new_mask);

            for m in masks.iter() {
                memory.insert(u64::from_str_radix(m, 2).unwrap(), number);
            }
        }
    }
    println!("Result 2: {}", memory.values().sum::<u64>());

    Ok(())
}


fn get_bit(number: u64, pos: usize) -> u8 {
    ((number >> pos) % 2) as u8
}

fn apply_mask(mask: &str, number: &mut u64) {
    mask.chars().rev().enumerate().for_each(|(i, c)| {
    if let Ok(bit) = c.to_string().parse::<u8>() {
        let current_bit = get_bit(*number, i);
        if current_bit != bit {
            if current_bit == 0 {
                *number += 1u64 << i; 
            } else {
                *number -= 1 << i;
            }
        }
    }
});

}

fn build_masks(mask: &str) -> Vec<String> {
    let mut masks : Vec<String> = Vec::new();
    let c = mask.chars().next().unwrap();
    let i;
    if c != 'X' {
        masks.push(c.to_string());
        i = 1;
    } else {
        masks.push("".to_owned());
        i = 0;
    }
    for c in mask.chars().skip(i) {
        if c != 'X' {
            for s in masks.iter_mut() {
                s.push(c);
            }
        } else {
            let v = masks.clone();
            masks.clear();
            for s in v {
                let m1 = s.to_owned() + "0";
                let m2 = s + "1";
                masks.push(m1);
                masks.push(m2);
            }
        }
    }
   masks
}