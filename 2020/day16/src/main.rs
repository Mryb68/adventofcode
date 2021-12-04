use std::io::Error;
fn main() -> Result<(), Error> {
    let my_ticket = [
        73, 53, 173, 107, 113, 89, 59, 167, 137, 139, 71, 179, 131, 181, 67, 83, 109, 127, 61, 79,
    ];
    let n = my_ticket.len();
    let rules = [
        ("departure location", [(27, 672), (680, 954)]),
        ("departure station", [(25, 430), (439, 966)]),
        ("departure platform", [(31, 293), (299, 953)]),
        ("departure track", [(29, 749), (775, 955)]),
        ("departure date", [(43, 93), (107, 953)]),
        ("departure time", [(50, 916), (941, 963)]),
        ("arrival location", [(31, 682), (702, 954)]),
        ("arrival station", [(38, 663), (672, 960)]),
        ("arrival platform", [(31, 629), (635, 969)]),
        ("arrival track", [(42, 365), (371, 967)]),
        ("class", [(30, 147), (167, 966)]),
        ("duration", [(39, 525), (545, 967)]),
        ("price", [(30, 803), (822, 950)]),
        ("route", [(39, 235), (257, 957)]),
        ("row", [(33, 206), (231, 971)]),
        ("seat", [(29, 784), (798, 951)]),
        ("train", [(38, 302), (316, 957)]),
        ("type", [(50, 635), (642, 966)]),
        ("wagon", [(25, 502), (510, 973)]),
        ("zone", [(42, 843), (861, 965)]),
    ];

    let tickets: Vec<Vec<usize>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    let mut ratio = 0;
    let mut valid_tickets = Vec::new();
    for ticket in tickets.iter() {
        if is_valid(&ticket, &rules, &mut ratio) {
            valid_tickets.push(ticket);
        }
    }
    println!("invalid ratio: {}", ratio);

    (0..n).for_each(|i| {
        for (r_i, r) in rules.iter().enumerate() {
            let mut match_rule = true;
            for t in valid_tickets.iter() {
                let number = t[i];
                if !belong_to_rule(number, r) {
                    match_rule = false;
                    break;
                }
            }
            if match_rule {
                println!("number index {} is valid with rule {}", i, r_i);
            }
        }
    });
    Ok(())
}

fn is_valid(ticket: &[usize], rules: &[(&str, [(usize, usize); 2])], ratio: &mut usize) -> bool {
    ticket.iter().all(|&number| {
        let any_valid = rules.iter().any(|(_, v)| {
            (number >= v[0].0 && number <= v[0].1)
                || (number >= v[1].0 && number <= v[1].1)
        });
        if !any_valid {
            *ratio += number;
        } 
        any_valid
    })
}

fn belong_to_rule(number: usize, rule: &(&str, [(usize, usize); 2])) -> bool {
    let (_, v) = rule;
    (number >= v[0].0 && number <= v[0].1) || (number >= v[1].0 && number <= v[1].1)
}
