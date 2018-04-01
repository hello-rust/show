use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::collections::{HashMap, HashSet};

fn main() {
    let numbers: Vec<i64> = (1..1000).collect();
    println!("{:?}", numbers)

    let path = "/usr/share/dict/words";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let words: Vec<String> =  buffered.lines().filter_map(Result::ok).collect();

    let mut word_length: HashMap<usize, _> = HashMap::new();
    for word in words {
        word_length.entry(word.len()).or_insert(HashSet::new()).insert(word);
    }

    println!("{:?}", word_length[&3]);
}
