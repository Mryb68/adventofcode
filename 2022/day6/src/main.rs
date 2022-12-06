use std::collections::HashSet;

fn get_marker(input: &[char], expected_len: usize) -> usize {
    for (index, w) in input.windows(expected_len).enumerate() {
        if w.iter().copied().collect::<HashSet<char>>().len() == expected_len {
            return index + expected_len;
        }
    }
    0
}

fn main() {
    let input: Vec<char> = include_str!("../input.txt").chars().collect();

    let result1 = get_marker(&input, 4);
    println!("Result 1: {}", result1);

    let result2 = get_marker(&input, 14);
    println!("Result 2: {}", result2);

}
