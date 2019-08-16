extern crate clap;

use clap::{Arg, App};

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
    let file = matches.value_of("file").unwrap();
    println!("{}", file);
}