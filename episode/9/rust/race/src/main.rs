extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum Error {
    Lock,
}

type Words = HashMap<String, u32>;

fn main() -> Result<(), Box<Error>> {
    let mut words = Words::new();
    env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .par_iter() // iter() works
        .for_each(|arg| tally_words(arg.to_string(), &mut words).unwrap());

    for (word, count) in words.iter() {
        if *count > 1 {
            println!("{} {}", count, word)
        }
    }

    Ok(())
}

fn tally_words(filename: String, w: &mut Words) -> Result<(), Box<Error>> {
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        *w.entry(key).or_insert(0) += 1;
    }
    Ok(())
}
