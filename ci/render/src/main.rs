#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate url;
extern crate url_serde;

use std::collections::HashMap;
use url::Url;
use url_serde::{deserialize, serialize};

use tera::{Context, Result, Tera};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Episode {
    number: u64,
    title: String,
    #[serde(with = "self")]
    url: url::Url,
    intro: String,
    details: String,
    keywords: Vec<String>,
    notes: Vec<String>,
    others: Option<Vec<String>>,
    metas: Option<Vec<String>>,
    markers: Option<Vec<String>>,
}

/// Render episode data in many different formats, yo!
#[derive(Debug, StructOpt)]
struct Cli {
    /// The input YAML file for the episode
    input: String,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = compile_templates!("ci/render/templates/**/*");
        tera
    };
}

main!(|cli: Cli| {
    let content = read_file(cli.input)?;
    let episode: Episode = serde_yaml::from_str(&content)?;
    let mut context = Context::new();
    context.add("episode", &episode);

    let readme = TEMPLATES.render("SHOW_NOTES.md", &context);
    write_to_file(
        format!("episode/{}/README.md", episode.number),
        &readme.expect("Cannot render README.md"),
    );

    let youtube = TEMPLATES.render("YOUTUBE.md", &context);
    write_to_file(
        format!("episode/{}/YOUTUBE.md", episode.number),
        &youtube.expect("Cannot render README.md"),
    );

    let slug = TEMPLATES.render("EPISODE_LIST.md", &context);
    println!("{}", slug.expect("Cannot render episode list entry"));
});
