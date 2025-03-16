use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 5);

    let mut file = File::create("scores.txt").unwrap();

    for (name, &score) in &scores {
        file.write_fmt(format_args!("{}: {}\n", name, score));
    }
}
