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
    others: Vec<String>,
    metas: Vec<String>,
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
    println!("{:?}", episode);

    let mut context = Context::new();
    context.add("episode", &episode);

    match TEMPLATES.render("SHOW_NOTES.md", &context) {
        Ok(s) => println!("{}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
});
