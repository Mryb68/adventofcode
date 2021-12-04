use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instr {
    None,
    Acc(i32),
    Jmp(i32)
}

fn main() -> Result<(), Error> {
    let mut instructions : Vec<Instr> = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                if l.starts_with("jmp") {
                    Instr::Jmp(l[4..].parse().unwrap())
                } else if l.starts_with("acc") {
                    Instr::Acc(l[4..].parse().unwrap())
                } else {
                    Instr::None
                }
            } else {
                Instr::None
            }
        }).collect();

    let mut pc;
    let mut acc;
    let mut seen = HashSet::new();
    let jmps : Vec<usize> = instructions.iter().enumerate().filter(|(_, e)| if let Instr::Jmp(_) = e { true } else { false }).map(|(i, _)| i).collect();
    for jmp_i in jmps {
        seen.clear();
        acc = 0;
        pc = 0;
        let inst = instructions[jmp_i];
        instructions[jmp_i] = Instr::None;
        loop {
            if !seen.insert(pc) {
                break;
            }
            match instructions[pc as usize] {
                Instr::Acc(n) => {
                    acc += n;
                    pc += 1;
                },
                Instr::Jmp(n) => {
                    pc += n;
                },
                Instr::None => {
                    pc += 1
                }
            };
            if pc as usize >= instructions.len() {
                println!("Exiting loop with Acc equals to {}", acc);
                return Ok(());
            }
        }
        instructions[jmp_i] = inst;
    }
    Ok(())
}