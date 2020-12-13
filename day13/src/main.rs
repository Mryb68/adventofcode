fn main() {
    let timestamp = 1_000_186;
    let bus_ids: [(i64, i64); 9] = [
        (17, 0),
        (37, 11),
        (907, 17),
        (19, 29),
        (23, 40),
        (29, 46),
        (653, 48),
        (41, 58),
        (13, 61),
    ];

    // Part 1
    let mut curr_time: i64 = timestamp;
    let mut bus_found = false;
    while !bus_found {
        for bus in bus_ids.iter() {
            if curr_time % bus.0 == 0 {
                println!(
                    "Closest timestamp found: {}",
                    (curr_time - timestamp) * bus.0
                );
                bus_found = true;
                break;
            }
        }
        curr_time += 1;
    }

    // Part 2 using chinese remainder theorem
    let product: i64 = bus_ids.iter().map(|n| n.0).product();
    let mut result: i64 = bus_ids
        .iter()
        .map(|(id, position)| {
            let n = product / id;
            bezout_coeff_u(n, *id) * n * ((-position) % id)
        })
        .sum::<i64>()
        % product;
    if result < 0 {
        result += product;
    }
    println!("timestamp: {}", result);
}

fn bezout_coeff_u(a: i64, b: i64) -> i64 {
    bezout_internal(a, b, 1, 0, 0, 1)
}

fn bezout_internal(r: i64, rp: i64, u: i64, up: i64, v: i64, vp: i64) -> i64 {
    if rp == 0 {
        u
    } else {
        let q = r / rp;
        bezout_internal(rp, r - q * rp, up, u - q * up, vp, v - q * vp)
    }
}
