use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input: Vec<Vec<Vec<String>>> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" | ")
                .map(|st| st.split_whitespace().map(|s| s.to_string()).collect())
                .collect()
        })
        .collect();

    let count_unique_digits = input.iter().fold(0, |acc, line| {
        acc + line[1]
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    });
    println!("Result 1: {}", count_unique_digits);

    let sum: usize = input
        .iter()
        .map(|l| {
            let code = compute_code(&l[0]);
            l[1].iter()
                .map(|out| {
                    get_digit_from_segment_list(
                        &out.chars()
                            .map(|c| *code.get(&c).unwrap())
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("Result 2: {}", sum);
}

static SEGMENTS_FOR_NUMBER: [&[usize]; 10] = [
    &[0, 1, 2, 4, 5, 6],    //0
    &[2, 5],                //1
    &[0, 2, 3, 4, 6],       //2
    &[0, 2, 3, 5, 6],       //3
    &[1, 2, 3, 5],          //4
    &[0, 1, 3, 5, 6],       //5
    &[0, 1, 3, 4, 5, 6],    //6
    &[0, 2, 5],             //7
    &[0, 1, 2, 3, 4, 5, 6], //8
    &[0, 1, 2, 3, 5, 6],    //9
];

fn get_digit_from_segment_list(segment_indexes: &[usize]) -> char {
    let mut segments_sorted = segment_indexes.to_owned();
    segments_sorted.sort_unstable();
    SEGMENTS_FOR_NUMBER
        .iter()
        .enumerate()
        .find(|(_, &s)| s == segments_sorted)
        .unwrap()
        .0
        .to_string()
        .chars()
        .next()
        .unwrap()
}

fn compute_code(inputs: &[String]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    // segment 0 is in digit 7 but not in digit 1
    let mut result = vec![None; 7];
    let digit_1 = inputs.iter().find(|&d| d.len() == 2).unwrap();
    let digit_7 = inputs.iter().find(|&d| d.len() == 3).unwrap();
    let digit_4 = inputs.iter().find(|&d| d.len() == 4).unwrap();
    let digit_len_5 = inputs.iter().filter(|&d| d.len() == 5);
    result[0] = digit_7
        .chars()
        .find(|&c| digit_1.chars().find(|&c2| c2 == c) == None);
    // segment 5 is in all digit but digit 2, so it appears nine times
    result[5] = "abcdefg".chars().find(|&c| {
        inputs
            .iter()
            .map(|s| s.chars())
            .flatten()
            .filter(|&c2| c2 == c)
            .count()
            == 9
    });
    // digit 2 contains segments 2 and 5
    result[2] = digit_1.chars().find(|&c| c != result[5].unwrap());
    // segment 1 is in digit_5, segment 4 is in digit 2. Both are unique in digit with length == 5. Moreover digit_4 contains segment_1 but not segment-4
    let flat_string: String = digit_len_5.map(|s| s.chars()).flatten().collect();
    let one_four: Vec<char> = flat_string
        .chars()
        .filter(|&c| flat_string.chars().filter(|&c2| c == c2).count() == 1)
        .collect();
    assert_eq!(one_four.len(), 2);
    if digit_4.chars().any(|c| c == one_four[0]) {
        result[1] = Some(one_four[0]);
        result[4] = Some(one_four[1]);
    } else {
        result[1] = Some(one_four[1]);
        result[4] = Some(one_four[0]);
    }
    // segment 3 is the segment not found from digit_4
    result[3] = digit_4
        .chars()
        .find(|&c| c != result[1].unwrap() && c != result[2].unwrap() && c != result[5].unwrap());
    // 6 is the  not used yet
    result[6] = "abcdefg".chars().find(|&c| {
        result
            .iter()
            .filter(|opt| opt.is_some() && opt.unwrap() == c)
            .count()
            == 0
    });

    for (segment, letter) in result.iter().enumerate() {
        map.insert(letter.unwrap(), segment);
    }
    map
}
