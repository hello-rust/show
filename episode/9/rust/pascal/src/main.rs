extern crate rayon;
#[macro_use] extern crate structopt;
extern crate im;

use rayon::prelude::*;
use structopt::StructOpt;
use im::ordmap::OrdMap;
use std::fs;
use std::path::{PathBuf, Path};

#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

#[derive(Debug)]
enum Error {
    Lock,
}

type Words = OrdMap<String, u32>;

fn main() -> Result<(), Box<Error>> {
    let mut words = Words::new();
    let args = Cli::from_args();

    args.files
        .par_iter()
        .for_each(|filename| tally_words(filename, &mut words).unwrap());

    for (word, count) in words.iter() {
        if *count > 1 {
            println!("{} {}", count, word)
        }
    }

    Ok(())
}

fn tally_words(filename: &Path, words: &mut Words) -> Result<(), Box<Error>> {
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    for s in contents.split_whitespace() {
        let key = s.to_lowercase();
        {
            *words.entry(key).or_insert(0) += 1;
        }
    }
    Ok(())
}
