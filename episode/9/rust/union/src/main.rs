extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

type Words = HashMap<String, u32>;

fn main() -> Result<(), Box<Error>> {
    let words = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .par_iter()
        .map(|arg| {
            tally_words(arg)
                .map_err(|e| eprintln!("Error processing {}: {:?}", arg, e))
                .unwrap_or_default()
        }).reduce(
            || Words::new(),
            |mut result, current| {
                for (key, val) in current {
                    result.entry(key).and_modify(|e| *e += val).or_insert(val);
                }
                result
            },
        );

    for (word, count) in words.iter() {
        if *count > 1 {
            println!("{} {}", count, word)
        }
    }

    Ok(())
}

fn tally_words(filename: &str) -> Result<Words, Box<Error>> {
    let mut words = Words::new();
    let contents = fs::read_to_string(filename)?;

    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        *words.entry(key).or_insert(0) += 1;
    }
    Ok(words)
}
