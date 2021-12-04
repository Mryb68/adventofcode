use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut allergens: HashMap<String, Vec<String>> = HashMap::new();
    let foods: Vec<Vec<String>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            let mut r = l.split('|');
            let ingredients: Vec<String> = r
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect();
            let allergen: Vec<String> = r
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect();
            for a in allergen.iter() {
                let al = allergens.get(a);
                if al.is_none() {
                    allergens.insert(a.to_owned(), ingredients.clone());
                } else {
                    let i_list = allergens.remove(a).unwrap();
                    let mut new_i_list: Vec<String> = Vec::new();
                    for i in i_list.iter() {
                        if contains(i, &ingredients) {
                            new_i_list.push(i.to_owned());
                        }
                    }
                    allergens.insert(a.to_owned(), new_i_list);
                }
            }
            ingredients
        })
        .collect();
    let keys: Vec<String> = allergens.keys().cloned().collect();
    loop {
        for k in keys.iter() {
            let v = allergens.get(k).unwrap().clone();
            if v.len() == 1 {
                let unique = v[0].clone();
                for a in allergens.values_mut() {
                    if a.len() > 1 && contains(&unique, a) {
                        a.remove(
                            a.iter()
                                .enumerate()
                                .find(|&(_, s)| s == &unique)
                                .map(|(i, _)| i)
                                .unwrap(),
                        );
                    }
                }
            }
        }
        if allergens.values().all(|v| v.len() == 1) {
            break;
        }
    }
    let mut allergenic_ingredients = HashSet::new();
    for ingredient in allergens.values() {
        allergenic_ingredients.insert(&ingredient[0]);
    }

    let result1: usize = foods
        .iter()
        .map(|ingredient_list| {
            ingredient_list
                .iter()
                .filter(|&ingredient| !allergenic_ingredients.contains(ingredient))
                .count()
        })
        .sum();
    println!("allergens: {:?}", allergens);
    println!("Result part 1: {}", result1);
    println!("Sort your allergens by hand to get result 2");
    let result2 = "fqhpsl,zxncg,clzpsl,zbbnj,jkgbvlxh,dzqc,ppj,glzb"; //dairy,eggs,fish,nuts,peanuts,sesame,soy,wheat
    println!("Result part 2: {}", result2);
    Ok(())
}

fn contains(item: &str, container: &[String]) -> bool {
    container.iter().any(|s| s == item)
}
