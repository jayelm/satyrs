extern crate tempfile;

use std::fmt::{ Display, Formatter, Error };
use std::iter::Iterator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::clone::Clone;

/**
 * test can be deleted.
 * clauses will house all clauses, key is int so that we can use occurrences
 * ID for simplifications later
 * occurrences tracks which clauses literals are used in
 */
#[derive(Debug)]
pub struct CNF {
    pub nvar        : i32,
    pub nclause     : i32,
    pub clauses     : HashMap<i32, Vec<i32>>,
    pub occurrences : HashMap<i32, Vec<i32>>
}

impl CNF {
    pub fn new(nvar: i32, nclause: i32) -> CNF {
        CNF {
            nvar        : nvar,
            nclause     : nclause,
            clauses     : HashMap::new(),
            occurrences : HashMap::new()
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
        format!("Nvar: {:?} Nclause: {:?}\nClauses: {:?}\nOccurrrences: {:?}",
                self.nvar, self.nclause,
                self.clauses, self.occurrences)
    }
}

impl Clone for CNF {
    fn clone(&self) -> CNF {
        CNF {
            nvar : self.nvar,
            nclause : self.nclause,
            clauses : self.clauses.clone(),
            occurrences : self.occurrences.clone(),
        }
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

// Begin Assignemnts 

pub type Assignment = Vec<bool>;

pub struct PartialAssignment {
    pub assignment : Vec<Option<bool>>,
    pub unassigned : HashSet<i32>,
}

impl PartialAssignment {
    pub fn new(n : usize) -> PartialAssignment {
        PartialAssignment {
            assignment : vec!(None; n),
            unassigned : (0..n as i32).collect(),
        }
    }

    pub fn assign(&mut self, v : usize, assn : bool){
        // TODO: Error check
        self.assignment[v] = Some(assn);
    }

    pub fn unassign(&mut self, v : usize) {
        self.assignment[v] = None;
    }

    pub fn to_string(self) -> String {
        format!("Assignemnts: {:?}\nUnassigned: {:?}",
                self.assignment, self.unassigned)
    }
}

// TODO: Implement Display
// impl Display for PartialAssignment {} 

// End Assignments

// Begin Parsing

#[derive(Debug)]
pub enum SatError {
    InvalidSyntax,
    InvalidProblem
}

fn parse_dimacs(reader: &mut BufReader<File>) -> Result<CNF, SatError> {
    let mut line_iterator = reader.lines();

    let mut nvar: i32 = -1;
    let mut nclause: i32 = -1;
    // DIMACS file must have a problem statement before other lines.
    // This first loop searches for the problem statement.
    for line in &mut line_iterator {
        let line = line.expect("Could not read file");  // Unwrap result
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "c" => { }
            "p" => { // Problem statement
                // Must have format "p cnf nvar nclause"
                if words.len() != 4 || words[1] != "cnf" {
                    return Err(SatError::InvalidSyntax);
                }
                nvar = words[2].parse()
                    .expect(&format!("Invalid number of variables {}", words[2]));
                nclause = words[3].parse()
                    .expect(&format!("Invalid number of clauses {}", words[3]));
                break;
            }
            _ => { return Err(SatError::InvalidSyntax); }
        }
    }
    // Then nvar, nclause were never initialized
    if nvar == -1 || nclause == -1 {
       return Err(SatError::InvalidProblem);
    }
    // TODO: Different errors for different descriptions (hence why I've split up this if
    // statement)
    if nvar == 0 || nclause == 0 {
        return Err(SatError::InvalidProblem)
    }

    // Initialize CNF and parse the rest of the file
    let mut cnf = CNF::new(nvar, nclause);
    for line in &mut line_iterator {
        let line = line.expect("Could not read filce");
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "c" => { }
            _   => {
                let tokens: Vec<i32> = words.iter()
                    .filter_map(|s| {
                        let n = s.parse::<i32>().expect("Invalid DIMACS File");
                        // FIXME: This ignores zeros not just as line enders but in the formulas
                        // themselves. TODO: Split on zeros at the end here.
                        if n == 0 { return None; }
                        if n > nvar { panic!(format!("Variable out of range: {}", n)); }
                        Some(if n < 0 { (-n) << 1 | 1 } else { n << 1 })
                    })
                    .collect();
                cnf.add_clause(tokens);
            }
        }
    }
    Ok(cnf)
}

pub fn parse_dimacs_file(f: File) -> Result<CNF, SatError> {
    // Read the file
    let mut reader = BufReader::new(f);

    // TODO: This is definitely not the correct way to handle errors.
    // Should parse_dimacs have options for returning an error AND panicking?
    let line = parse_dimacs(&mut reader);
    line
}

#[cfg(test)]
mod tests {
	extern crate tempfile;
	use std::fs::File;
	use std::io::{Write, Seek, SeekFrom};

    use super::parse_dimacs_file;

    macro_rules! create_tempfile {
        ($x: expr) => {{
            let mut tmpfile: File = tempfile::tempfile().unwrap();
            let _ = write!(tmpfile, $x);
            tmpfile.seek(SeekFrom::Start(0)).unwrap();

            tmpfile
        }};
    }

    #[test]
    #[should_panic]
    fn variable_out_of_range() {
        let tmpfile = create_tempfile!("
            p cnf 2 3
            1 2 0
            4 1 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }
}
