extern crate clap;
use clap::{Arg, App};

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "ck2txt.pest"]
pub struct CK2Parser;

use std::io::prelude::*;
use std::fs::File;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use pest::error::Error;

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)|
                     format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = CK2Parser::parse(Rule::file, file)?.skip(1).next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            //Rule::header => JSONValue::Object(vec![(pair.as_str(), parse_value(pair.into_inner().next().unwrap()))]),
            Rule::object
            | Rule::body => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string
            | Rule::date => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str() == "yes"),
            //Rule::null => JSONValue::Null,
            Rule::file
            | Rule::EOI
            | Rule::header
            | Rule::identifier
            | Rule::tag
            | Rule::checksum
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::int
            | Rule::float
            | Rule::date_inner
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

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

    let json: JSONValue = parse_json_file(&file_text).expect("unsuccessful parse");

    println!("{}", serialize_jsonvalue(&json));

    let pairs = CK2Parser::parse(Rule::file, &file_text).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::identifier => println!("ok"),
                // Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                // Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}