use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use anyhow::{Context, Result, anyhow};
use tera::Tera;
use tera::Context as TeraContext;

use std::env;
use std::path::{Path};
use std::io::{ BufWriter, Write};
use std::fs::{self, File};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Episode {
    number: u64,
    title: String,
    id: String,
    intro: String,
    details: Option<String>,
    keywords: Vec<String>,
    notes: Vec<String>,
    others: Option<Vec<String>>,
    errata: Option<Vec<String>>,
    metas: Option<Vec<String>>,
    licenses: Option<Vec<String>>,
    markers: Option<Vec<String>>,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("ci/render/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();

    let file =
        File::create(path).with_context(|| format!("Could not create/open file {:?}", path))?;
    let mut file = BufWriter::new(file);

    file.write_all(content.as_bytes())
        .with_context(|| format!("Could not write to file {:?}", path))?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Expected exactly on argument: <path/to/episode.yml>"));
    }

    let content = fs::read_to_string(args[1].clone())?;
    let episode: Episode = serde_yaml::from_str(&content)?;
    let mut context = TeraContext::new();
    context.insert("episode", &episode);

    let readme = TEMPLATES.render("SHOW_NOTES.md", &context);
    write_to_file(
        format!("episode/{}/README.md", episode.number),
        &readme.context("Cannot render README.md")?,
    )?;

    fs::create_dir_all(format!("episode/{}/meta", episode.number))?;

    let readme = TEMPLATES.render("WEBSITE.md", &context);
    write_to_file(
        format!("episode/{}/meta/index.md", episode.number),
        &readme.context("Cannot render WEBSITE.md")?,
    )?;

    let youtube = TEMPLATES.render("YOUTUBE.md", &context);
    write_to_file(
        format!("episode/{}/meta/YOUTUBE.md", episode.number),
        &youtube.context("Cannot render YouTube description")?,
    )?;

    let youtube_title = TEMPLATES.render("YOUTUBE_TITLE.md", &context);
    write_to_file(
        format!("episode/{}/meta/YOUTUBE_TITLE.md", episode.number),
        &youtube_title.context("Cannot render YouTube title")?,
    )?;

    let tweet = TEMPLATES.render("TWEET.md", &context);
    write_to_file(
        format!("episode/{}/meta/TWEET.md", episode.number),
        &tweet.context("Cannot render tweet")?,
    )?;

    let slug = TEMPLATES.render("EPISODE_LIST.md", &context);
    println!("{}", slug.context("Cannot render episode list entry")?);
    Ok(())
}
