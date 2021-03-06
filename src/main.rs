extern crate clap;
extern crate env_logger;
extern crate jst;

use clap::{App, Arg};
use jst::{Context, ResolveMethod};
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
    env_logger::init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("file")
                .help("Sets the input file to use")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("type")
                .help("The output schema format")
                .short("t")
                .long("type")
                .takes_value(true)
                .possible_values(&["avro", "bigquery"])
                .default_value("avro"),
        )
        .arg(
            Arg::with_name("resolve")
                .help("The resolution strategy for incompatible or under-specified schema")
                .short("r")
                .long("resolve")
                .takes_value(true)
                .possible_values(&["cast", "panic", "drop"])
                .default_value("cast"),
        )
        .arg(
            Arg::with_name("normalize-case")
                .help("snake_case column-names for consistent behavior between SQL engines")
                .short("c")
                .long("normalize-case"),
        )
        .get_matches();

    let reader: Box<io::Read> = match matches.value_of("file") {
        Some(path) if path == "-" => Box::new(io::stdin()),
        Some(path) => {
            let file = File::open(path).unwrap();
            Box::new(BufReader::new(file))
        }
        None => Box::new(io::stdin()),
    };
    let data: Value = serde_json::from_reader(reader).unwrap();
    let context = Context {
        resolve_method: match matches.value_of("resolve").unwrap() {
            "cast" => ResolveMethod::Cast,
            "panic" => ResolveMethod::Panic,
            "drop" => ResolveMethod::Drop,
            _ => panic!("Unknown resolution method!"),
        },
        normalize_case: matches.is_present("normalize-case"),
    };

    let output = match matches.value_of("type").unwrap() {
        "avro" => jst::convert_avro(&data, context),
        "bigquery" => jst::convert_bigquery(&data, context),
        _ => panic!("Unknown type!"),
    };
    let pretty = serde_json::to_string_pretty(&output).unwrap();
    println!("{}", pretty);
}
