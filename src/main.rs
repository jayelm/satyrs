extern crate argparse;

use std::fs::File;
use argparse::{ArgumentParser, Store};
use satyrs::cnf::CNF;
use satyrs::dpll;

//extern crate satyrs;
mod satyrs;

fn main() {
    let mut filename = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Satyrs: A lustful, drunken SAT solver");
        ap.refer(&mut filename)
            .add_argument(
                "filename",
                Store,
                "SAT filename to read")
            .required();
        ap.parse_args_or_exit();
    }

    // Read the file
    let f : File = File::open(filename)
        .expect("Could not open file");

    // TODO: This is definitely not the correct way to handle errors
    let cnf : CNF = satyrs::cnf::parse_dimacs_file(f).expect("Dimacs Error");
    let solvable = dpll::DPLL(&cnf);
    println!("{}", cnf);
    match solvable {
        Some(solution)=> println!("Solution: {:?}",solution),
        None => println!("No solution"),
    }
}
