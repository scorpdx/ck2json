extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate pest_derive;

use std::io::prelude::*;
use std::fs::File;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod json;
use json::*;

mod ck2json;
use ck2json::*;

fn main() {
    let matches = App::new("ck2json")
        .version("0.1.0")
        .author("J. Zebedee <zebedee@code.gripe>")
        .about("Convert CK2txt-format files to JSON")
        .arg(Arg::with_name("file")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("CK2txt-format file to parse"))
        .arg(Arg::with_name("grammar")
                 .possible_values(&["ck2txt", "cultures"])
                 .default_value("ck2txt"))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let file = File::open(filename).expect("cannot open file");

    let mut transcoded = DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(file);

    let mut file_text = String::new();
    transcoded.read_to_string(&mut file_text).expect("cannot transcode file");

    let grammar_name = matches.value_of("grammar").unwrap();
    let json = match grammar_name {
        "ck2txt" => ck2parser::parse(&file_text).expect("unsuccessful parse"),
        "cultures" => cultureparser::parse(&file_text).expect("unsuccessful parse"),
        _ => unreachable!("unknown grammar type")
    };

    println!("{}", serialize_jsonvalue(&json));
}