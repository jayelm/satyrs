extern crate argparse;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use argparse::{ArgumentParser, Store};

#[derive(Debug)]
enum SatError {
    InvalidSyntax
}

fn parse_dimacs(reader: &mut BufReader<std::fs::File>) -> Result<String, SatError> {
    for line in reader.lines() {
        let line = line.expect("Could not read file");
        match &line[..2] {
            "c " => println!("Comment"), // Comment, ignore
            "p " => println!("Problem statement"), // Problem statement
            _    => return Err(SatError::InvalidSyntax)
        }
    }
    Ok("No errors".into())
}

fn main() {
    let mut filename = "".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Satyrs: A lustful, drunken SAT solver");
        ap.refer(&mut filename)
            .add_argument(
                "filename",
                Store,
                "SAT filename to read"
            ).required();
        ap.parse_args_or_exit();
    }

    // Read the file
    let f = File::open(filename)
        .expect("Could not open file");
    let mut reader = BufReader::new(f);

    // TODO: This is definitely not the correct way to handle errors
    let line = parse_dimacs(&mut reader).expect("Parse error");

    println!("{}", line)
}
