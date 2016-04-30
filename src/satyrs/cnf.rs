use std::fmt::{ Display, Formatter, Error };
use std::iter::Iterator;
use std::collections::HashMap;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/**
 * test can be deleted.
 * clauses will house all clauses, key is int so that we can use occurrences
 * ID for simplifications later
 * occurrences tracks which clauses literals are used in
 */
pub struct CNF {
    clauses     : HashMap<i32,Vec<i32>>,
    occurrences : HashMap<i32,Vec<i32>>,
}

impl CNF {
    pub fn new() -> CNF {
        CNF {
            test : "NEW".to_string(),
            clauses : HashMap::new(),
            occurrences : HashMap::new(),
        }
    }

    // Add a clause, return the ID of the inserted clause
    fn add_clause(&mut self, clause : Vec<i32>) -> i32 {
        let id : i32 = self.clauses.len() as i32;
        for var in &clause {
            let occ = self.occurrences.entry(*var).or_insert(Vec::new());
            occ.push(id);
        }
        self.clauses.insert(id, clause);
        id
    }

    pub fn to_string(self) -> String {
        format!("Clauses: {:?}\nOccurrrences: {:?}",self.clauses,self.occurrences)
    }
}

impl Display for CNF {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();
        comma_separated.push_str("Need to implement formatter");
        write!(f, "{}", comma_separated)
    }
}

// End CNF

// Begin Parsing

#[derive(Debug)]
pub enum SatError {
    InvalidSyntax
}

fn parse_dimacs(reader: &mut BufReader<File>) -> Result<CNF, SatError> {
    let mut cnf = CNF::new();
    for line in reader.lines() {
        let line = line.expect("Could not read file");  // Unwrap result
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "c" => println!("Comment"), // Comment, ignore
            "p" => println!("Problem statement"), // Problem statement
            _   => {
                let tokens = words.iter()
                    .map(|s| s.parse().expect("Invalid DMACS File"))
                    .collect();;
                println!("{:?}", tokens);
                cnf.add_clause(tokens);
            }
        }
    }
    Ok(cnf)
}

pub fn parse_dimacs_file(f: File) -> Result<CNF,SatError> {
    // Read the file
    let mut reader = BufReader::new(f);

    // TODO: This is definitely not the correct way to handle errors
    let line = parse_dimacs(&mut reader);
    line
}
