use std::io::Error;
fn main() -> Result<(), Error> {
    let result1: u64 = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| evaluate(l))
        .sum();
    println!("result1: {}", result1);
    // 30753705453324

    let result2: u64 = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| evaluate_priority(l))
        .sum();
    println!("result2: {}", result2);
    // 244817530095503

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Token {
    Num(u64),
    Sum,
    Mul,
    ParOp,
    ParClose,
}

fn collect(line: &str) -> Vec<Token> {
    line.split(' ')
        .map(|s| match s.chars().next().unwrap() {
            '*' => Token::Mul,
            '+' => Token::Sum,
            '(' => Token::ParOp,
            ')' => Token::ParClose,
            _ => Token::Num(s.parse::<u64>().unwrap()),
        })
        .collect()
}

fn evaluate(line: &str) -> u64 {
    evaluate_expr(&collect(line))
}

fn evaluate_priority(line: &str) -> u64 {
    let mut tokens = collect(line);

    // Transforming input to add parenthesis where needed
    let mut i = 0;
    let mut n = tokens.len();
    while i < n {
        if tokens[i] == Token::Mul {
            let mut j = i - 1;
            let mut nb_close = 0;
            loop {
                if j == 0 {
                    break;
                }
                if tokens[j] == Token::ParClose {
                    nb_close += 1;
                } else if tokens[j] == Token::ParOp {
                    if nb_close == 0 {
                        break;
                    } else {
                        nb_close -= 1;
                    }
                }
                j -= 1
            }
            tokens.insert(i, Token::ParClose);
            tokens.insert(j, Token::ParOp);
            i += 3;
            j = i;
            let mut nb_open = 0;
            loop {
                if j > n {
                    break;
                }
                if tokens[j] == Token::ParOp {
                    nb_open += 1;
                } else if tokens[j] == Token::ParClose {
                    if nb_open == 0 {
                        break;
                    } else {
                        nb_open -= 1;
                    }
                }
                j += 1
            }
            tokens.insert(j + 1, Token::ParClose);
            tokens.insert(i, Token::ParOp);
        }
        i += 1;
        n = tokens.len();
    }

    evaluate_expr(&tokens)
}

fn evaluate_expr(expr: &[Token]) -> u64 {
    if expr.is_empty() {
        0
    } else {
        let mut result = 0;
        let mut sum = true;
        let mut i = 0;
        while i < expr.len() {
            match expr[i] {
                Token::Num(n) => {
                    if sum {
                        result += n;
                    } else {
                        result *= n;
                    }
                }
                Token::Sum => sum = true,
                Token::Mul => sum = false,
                Token::ParOp => {
                    let j = find_closing(&expr[i + 1..]);
                    let r = evaluate_expr(&expr[i + 1..i + 1 + j]);
                    if sum {
                        result += r;
                    } else {
                        result *= r;
                    }
                    i += j + 2;
                    continue;
                }
                Token::ParClose => {
                    panic!();
                }
            }
            i += 1;
        }
        result
    }
}

fn find_closing(expr: &[Token]) -> usize {
    let mut number_to_find = 1;
    for (i, t) in expr.iter().enumerate() {
        if *t == Token::ParClose {
            number_to_find -= 1;
            if number_to_find == 0 {
                return i;
            }
        } else if *t == Token::ParOp {
            number_to_find += 1;
        }
    }
    expr.len()
}
