use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input: Vec<Vec<String>> = std::io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().split('-').map(|s| s.to_string()).collect())
        .collect();

    let mut links: HashMap<String, Vec<&str>> = HashMap::new();
    // Initialise links. Dont put links toward start and links starting from end because they are forbidden
    for link in input.iter() {
        let (a, b) = (&link[0], &link[1]);
        if b != "start" {
            if let Some(v) = links.get_mut(a) {
                v.push(b);
            } else {
                links.insert(a.clone(), vec![b; 1]);
            }
        }
        if a != "start" {
            if let Some(v) = links.get_mut(b) {
                v.push(a);
            } else {
                links.insert(b.clone(), vec![a; 1]);
            }
        }
    }
    links.remove(&"end".to_string());

    println!("Result 1: {}", Node::new().visit_paths(&links, false));
    println!("Result 2: {}", Node::new().visit_paths(&links, true));
}

struct Node<'a> {
    value: &'a str,
    next: Vec<Node<'a>>,
    visited: HashSet<&'a str>,
    has_visited_twice: bool,
}

impl<'a> Node<'a> {
    pub fn new() -> Self {
        Node {
            value: "start",
            next: Vec::new(),
            visited: HashSet::new(),
            has_visited_twice: false,
        }
    }

    pub fn with_value(value: &'a str, visited: HashSet<&'a str>, has_visited_twice: bool) -> Self {
        Node {
            value,
            next: Vec::new(),
            visited,
            has_visited_twice,
        }
    }

    pub fn visit_paths(
        &mut self,
        links: &'a HashMap<String, Vec<&str>>,
        allow_visit_twice: bool,
    ) -> usize {
        if self.value == "end" {
            1
        } else {
            for name in links.get(&self.value.to_owned()).unwrap() {
                let mut new_set = self.visited.clone();
                //TODO: Find a way to avoid copying the visited map for each node
                if name.to_uppercase() != *name {
                    if self.visited.get(name).is_none() {
                        new_set.insert(name);
                        self.next
                            .push(Node::with_value(name, new_set, self.has_visited_twice));
                    } else if !self.has_visited_twice && allow_visit_twice {
                        new_set.insert(name);
                        self.next.push(Node::with_value(name, new_set, true));
                    }
                } else {
                    self.next
                        .push(Node::with_value(name, new_set, self.has_visited_twice));
                }
            }
            self.next
                .iter_mut()
                .map(|node| node.visit_paths(links, allow_visit_twice))
                .sum()
        }
    }
}
