extern crate clap;
use clap::{Arg, App};

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "ck2txt.pest"]
pub struct CK2Parser;

//use pest::error::Error;

use std::io::prelude::*;
use std::fs::File;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main() {
    let matches = App::new("ck2parser")
        .version("0.1.0")
        .author("J. Zebedee <zebedee@code.gripe>")
        .about("CK2 grammar parser in Rust")
        .arg(Arg::with_name("file")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("CK2txt-format file to parse"))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    println!("{}", filename);
    let file = File::open(filename).expect("cannot open file");

    let mut transcoded = DecodeReaderBytesBuilder::new().encoding(Some(WINDOWS_1252)).build(file);

    let mut file_text = String::new();
    transcoded.read_to_string(&mut file_text).expect("cannot transcode file");

    let pairs = CK2Parser::parse(Rule::file, &file_text).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
/*         for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        } */
    }
}