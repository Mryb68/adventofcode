use std::collections::HashMap;
enum  DirInfo {
    File((String, usize)),
    Dir(String)
}

struct Directory {
    parent: String,
    children: Vec<DirInfo>,
}

impl Directory {
    pub fn new(parent: &str) -> Self {
        Directory { parent: parent.to_owned(), children: Vec::new() }
    }
}

fn main() {
    let input: Vec<String> = include_str!("../input.txt").lines().map(|l| l.to_owned()).collect();

    let mut tree: HashMap<String, Directory> = HashMap::new();
    tree.insert("/".to_owned(), Directory::new(""));
}