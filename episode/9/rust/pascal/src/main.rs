extern crate rayon;
#[macro_use]
extern crate structopt;
extern crate im;

use im::HashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

type Words = HashMap<String, u32>;

fn main() -> Result<(), Box<Error>> {
    let args = Cli::from_args();

    let words = args
        .files
        .par_iter()
        .map(|filename| {
            tally_words(filename)
                .map_err(|e| eprintln!("Error processing {}: {:?}", filename.display(), e))
                .unwrap_or_default()
        }).reduce(
            || Words::new(),
            |result, current| current.union_with(result, |a, b| a + b),
        );

    for (word, count) in &words {
        if *count > 1 {
            println!("{} {}", count, word)
        }
    }

    Ok(())
}

fn tally_words(filename: &Path) -> Result<Words, Box<Error>> {
    let mut words = Words::new();
    let contents = fs::read_to_string(filename)?;

    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        *words.entry(key).or_insert(0) += 1;
    }
    Ok(words)
}
