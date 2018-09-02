extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Mutex;

#[derive(Debug)]
enum Error {
    Lock,
}

type Words = Mutex<HashMap<String, u32>>;

fn main() -> Result<(), Box<Error>> {
    let w = Words::new(HashMap::new());
    env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .par_iter()
        .for_each(|arg| tally_words(arg.to_string(), &w).unwrap());

    let words = w.lock().map_err(|_| Error::Lock)?;
    for (word, count) in words.iter() {
        if *count > 1 {
            println!("{} {}", count, word)
        }
    }

    Ok(())
}

fn tally_words(filename: String, w: &Words) -> Result<(), Box<Error>> {
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        {
            let mut map = w.lock().map_err(|_| Error::Lock)?;
            *map.entry(key).or_insert(0) += 1;
        }
    }
    Ok(())
}
