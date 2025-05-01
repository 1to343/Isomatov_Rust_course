#![forbid(unsafe_code)]
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};
// TODO: your code goes here.

fn read_file(path: &str) -> HashSet<String> {
    let mut lines: HashSet<String> = HashSet::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.insert(line.unwrap());
    }
    lines
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let first = read_file(&args[1]);
    let second = read_file(&args[2]);
    let _ = first.intersection(&second).for_each(|common| {
        println!("{}", common);
    });

    // TODO: your code goes here.
    // unimplemented!()
}
