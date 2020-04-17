#![feature(proc_macro_hygiene)]

extern crate clap;

use schemafy_core;
use serde::{Deserialize, Serialize};

use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("Json Schema to rust source generator.")
        .version("0.5.0")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("schema")
            .short("s")
            .long("schema")
            .takes_value(true)
            .help("Input json schema"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Output file"))
        .get_matches();

    let schema_file = matches.value_of("schema").unwrap_or("src/schema.json");
    let output_file = matches.value_of("output").unwrap_or("src/schema.rs");

    schemafy::generate_schema(schema_file.to_string(), output_file.to_string());
}

