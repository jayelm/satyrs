extern crate tempfile;

use std::fmt::{Display, Formatter, Error};
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::clone::Clone;

/// Get the (arbitrary) zeroth element of a hashset.
#[marco_export]
macro_rules! zeroth {
    ($hs: expr) => {{
        *$hs.iter().next().unwrap()
    }}
}

macro_rules! create_tempfile {
    ($x: expr) => {{
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        let _ = write!(tmpfile, $x);
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        tmpfile
    }};
}

/// CNF will house all clauses, key is int so that we can use occurrences
/// Occurrences tracks which clauses literals are used in for simplifications
/// in DPLL.
#[derive(Debug)]
pub struct CNF {
    pub nvar: i32,
    pub nclause: i32,
    pub clauses: HashMap<i32, HashSet<i32>>,
    pub occurrences: HashMap<i32, HashSet<i32>>,
    pub units: HashSet<i32>,
}

impl CNF {
    /// Create a new CNF using the p specifications in the DIMACS file.
    /// Number of variables and clauses must be known.
    /// If using add_clause, set `nclause` to 0.
    pub fn new(nvar: i32, nclause: i32) -> CNF {
        CNF {
            nvar: nvar,
            nclause: nclause,
            clauses: HashMap::new(),
            occurrences: HashMap::new(),
            units: HashSet::new(),
        }
    }

    /// The user friendly function for adding a clause to the CNF.
    /// Still requires that the CNF be created with proper number of variables
    #[allow(dead_code)]
    pub fn add_clause(&mut self, clause: Vec<i32>) {
        let hs: HashSet<i32> =
            clause.into_iter()
                  .filter_map(|n| {
                      // FIXME: This ignores zeros not just as line enders but in the formulas
                      // themselves. Split on zeros at the end here.
                      if n == 0 {
                          return None;
                      }
                      // FIXME: This should return an error instead of
                      // panicking, but I can't return an error
                      // in a closure
                      if n > self.nvar {
                          panic!("variable out of range: {}", n);
                      }
                      Some(if n < 0 {
                          (-n) << 1 | 1
                      } else {
                          n << 1
                      })
                  })
                  .collect();
        self.nclause += 1;
        self._add_clause(hs);
    }

    /// Add a clause, return the ID of the inserted clause
    /// Right now, this isn't public; api is odd as we have an odd representation of literals.
    /// TODO: Mask this with public function?
    fn _add_clause(&mut self, clause: HashSet<i32>) -> i32 {
        assert!(clause.len() > 0);
        let id: i32 = self.clauses.len() as i32;
        if clause.len() == 1 {
            self.units.insert(id);
        }
        for var in &clause {
            let occ = self.occurrences.entry(*var).or_insert(HashSet::new());
            occ.insert(id);
        }
        self.clauses.insert(id, clause);
        id
    }

    /// Unit propagation: propagate the literal associated with the clause id `clause`, and remove
    /// it from `self.units`.
    pub fn unit_propagate(&mut self, unit: i32) {
        // Remove clauses with lit and remove lit from occurrences.
        let clause = match self.clauses.get(&unit) {
            Some(x) => {
                if !x.is_empty() {
                    Some(zeroth!(x))
                } else {
                    None
                }
            }
            None => None,
        };
        if let Some(c) = clause {
            // TODO: Return here for empty clause
            self.propagate(c);
        }
        // This needs to be true, i.e. unit clause id needs to be in the unit clauses
        assert!(self.units.remove(&unit));
    }

    /// Remove all clauses containing literal `lit` from the CNF, and remove the negation of `lit`
    /// from the remaining clauses.
    /// Ex: (p^q^r)v(~q). Assign q false and fix equation to (p^r)
    /// TODO: unit_propagate and propagate should return true/false
    /// depending on whether the updated CNF problem is satisfiable
    pub fn propagate(&mut self, lit: i32) {
        // Remove clauses with lit and remove lit from occurrences.
        if let Some(vec) = self.occurrences.remove(&lit) {
            for occ in &vec {
                if let Some(lits) = self.clauses.remove(occ) {
                    for lit in lits {
                        // Book keeping on the occurrences of lit
                        if let Some(mut lit_occ) = self.occurrences.remove(&lit) {
                            lit_occ.remove(occ);
                            if !lit_occ.is_empty() {
                                self.occurrences.insert(lit, lit_occ);
                            }
                        }
                    }
                }
                self.nclause -= 1;
            }
        }

        // Remove ~lit from other clausesa
        self.remove_negation(lit);
    }

    /// Since we know the assignment of `lit`, we can remove the negation `~lit` from all other
    /// clauses.
    fn remove_negation(&mut self, lit: i32) {
        let neg = lit ^ 1;
        if let Some(vec) = self.occurrences.remove(&neg) {
            for occ in &vec {
                if let Some(mut c) = self.clauses.remove(occ) {
                    c.remove(&neg);
                    if c.len() == 1 {
                        self.units.insert(*occ);
                    }
                    self.clauses.insert(*occ, c);
                }
            }
        }
    }
}

/// Cloning the CNF is necessary for our current implementation. In a better
/// implementation we would try to avoid the need for cloning
impl Clone for CNF {
    fn clone(&self) -> CNF {
        CNF {
            nvar: self.nvar,
            nclause: self.nclause,
            clauses: self.clauses.clone(),
            occurrences: self.occurrences.clone(),
            units: self.units.clone(),
        }
    }
}

impl Display for CNF {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut fmt_clauses: HashMap<i32, Vec<i32>> = HashMap::new();
        for clause in self.clauses.iter() {
            let (id, cl) = clause;
            fmt_clauses.insert(*id,
                               cl.iter()
                                 .map(|l| if l % 2 == 0 {
                                     l / 2
                                 } else {
                                     -l / 2
                                 })
                                 .collect());
        }
        let mut fmt_occ: HashMap<i32, &HashSet<i32>> = HashMap::new();
        for occ in self.occurrences.iter() {
            let (id, oc) = occ;
            fmt_occ.insert(if *id % 2 == 0 {
                               *id / 2
                           } else {
                               -*id / 2
                           },
                           oc);
        }
        let formatted = format!("Nvar: {:?} Nclause: {:?} Units: {:?}\nClauses: \
                                 {:?}\nOccurrences: {:?}",
                                self.nvar,
                                self.nclause,
                                self.units,
                                fmt_clauses,
                                fmt_occ);
        write!(f, "{}", formatted)
    }
}

// End CNF

// Begin Assignments

pub type Assignment = Vec<bool>;

pub struct PartialAssignment {
    pub assignment: Vec<Option<bool>>,
    pub unassigned: HashSet<i32>,
}

impl PartialAssignment {
    pub fn new(n: usize) -> PartialAssignment {
        PartialAssignment {
            assignment: vec!(None; n),
            unassigned: (0..n as i32).collect(),
        }
    }

    fn assign(&mut self, v: usize, assn: bool) {
        // TODO: Error check
        assert!(self.assignment[v].is_none());
        self.assignment[v] = Some(assn);
        self.unassigned.remove(&(v as i32));
    }

    pub fn assign_literal(&mut self, lit: i32) {
        let polarity: bool = lit & 1 == 0;
        let v: usize = (lit >> 1) as usize;
        // println!("ASSIGN {}", v);
        self.assign(v - 1, polarity);
    }

    fn unassign(&mut self, v: usize) {
        assert!(self.assignment[v].is_some());
        self.assignment[v] = None;
        self.unassigned.insert(v as i32);
    }

    pub fn unassign_literal(&mut self, lit: i32) {
        let v: usize = (lit >> 1) as usize;
        // println!("UNASSIGN {}", v);
        self.unassign(v - 1);
    }
}

impl Clone for PartialAssignment {
    fn clone(&self) -> PartialAssignment {
        PartialAssignment {
            assignment: self.assignment.clone(),
            unassigned: self.unassigned.clone(),
        }
    }
}

// TODO: Implement Display
impl Display for PartialAssignment {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut var = 0;
        let assign_fmt: Vec<(i32, Option<bool>)> = self.assignment
                                                       .iter()
                                                       .map(|l| {
                                                           var += 1;
                                                           (var, *l)
                                                       })
                                                       .collect();
        let unassn_fmt: HashSet<i32> = self.unassigned.iter().map(|l| l + 1).collect();
        let formatted = format!("Assignments: {:?}\nUnassigned: {:?}",
                                assign_fmt,
                                unassn_fmt);
        write!(f, "{}", formatted)
    }
}

// End Assignments

// Begin Parsing

// TODO: rename to from_dimacs or somehow isolate parsing
fn parse_dimacs(reader: &mut BufReader<File>) -> Result<CNF, &'static str> {
    let mut line_iterator = reader.lines();

    let mut nvar: i32 = -1;
    let mut nclause: i32 = -1;
    // DIMACS file must have a problem statement before other lines.
    // This first loop searches for the problem statement.
    for line in &mut line_iterator {
        let line = line.expect("could not read file");  // Unwrap result
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 0 {
            continue;
        } // Ignore empty lines
        match words[0] {
            "c" => {}
            "p" => {
                // Problem statement
                // Must have format "p cnf nvar nclause"
                if words.len() != 4 || words[1] != "cnf" {
                    return Err("invalid problem statement");
                }
                nvar = words[2]
                           .parse()
                           .expect(&format!("invalid number of variables {}", words[2]));
                nclause = words[3]
                              .parse()
                              .expect(&format!("invalid number of clauses {}", words[3]));
                break;
            }
            // TODO: Add words[0] to this error message
            _ => {
                return Err("unknown statement beginning");
            }
        }
    }
    // Then nvar, nclause were never initialized
    if nvar == -1 || nclause == -1 {
        return Err("no problem statement found");
    }
    // TODO: Different errors for different descriptions (hence why I've split up this if
    // statement)
    if nvar == 0 || nclause == 0 {
        return Err("invalid number of literals in problem");
    }

    // Initialize CNF and parse the rest of the file
    let mut cnf = CNF::new(nvar, nclause);
    let mut clauses_read: i32 = 0;
    for line in &mut line_iterator {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 0 {
            continue;
        }
        match words[0] {
            "c" => {}
            "p" => {
                return Err("duplicate problem statement");
            }
            _ => {
                clauses_read = clauses_read + 1;
                if clauses_read > nclause {
                    return Err("too many clauses in file");
                }
                let tokens: HashSet<i32> = words.iter()
                                                .filter_map(|s| {
                                                    let n = s.parse::<i32>().unwrap();
                                                    if n == 0 {
                                                        return None;
                                                    }
                                                    // FIXME: This should return an error instead of
                                                    // panicking, but I can't return an error
                                                    // in a closure
                                                    if n > nvar {
                                                        panic!("variable out of range: {}", n);
                                                    }
                                                    Some(if n < 0 {
                                                        (-n) << 1 | 1
                                                    } else {
                                                        n << 1
                                                    })
                                                })
                                                .collect();
                cnf._add_clause(tokens);
            }
        }
    }
    // Double check that the number of clauses read is equal
    if clauses_read != nclause {
        return Err("too few clauses in file");
    }
    Ok(cnf)
}

pub fn parse_dimacs_file(f: File) -> Result<CNF, &'static str> {
    // Read the file
    let mut reader = BufReader::new(f);
    parse_dimacs(&mut reader)
}

pub fn format_output(assn: &Assignment) -> String {
    let mut output = String::new();
    let mut var: i32 = 0;
    for l in assn {
        var += 1;
        if *l {
            output.push_str(var.to_string().as_str());
        } else {
            output.push_str((-var).to_string().as_str());
        }
        if var < assn.len() as i32 {
            output.push(' ');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::SeekFrom;
    use std::collections::HashSet;

    use super::parse_dimacs_file;
    // use super::zeroth;

    #[test]
    #[should_panic(expected = "out of range")]
    fn variable_out_of_range() {
        let tmpfile = create_tempfile!("
            p cnf 2 3
            1 2 0
            4 1 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid number")]
    fn invalid_nvar() {
        let tmpfile = create_tempfile!("
            p cnf gd 3
            1 2 0
            4 1 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid number")]
    fn invalid_nclause() {
        let tmpfile = create_tempfile!("
            p cnf 2 gdd
            1 2 0
            4 1 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "too many")]
    fn too_many_clauses() {
        let tmpfile = create_tempfile!("
            p cnf 5 5
            1 2 0
            2 3 0
            3 4 0
            4 5 0
            1 3 0
            2 4 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "too few")]
    fn too_little_clauses() {
        let tmpfile = create_tempfile!("
            p cnf 5 5
            1 2 0
            2 3 0
            3 4 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    fn should_work() {
        let tmpfile = create_tempfile!("
            p cnf 3 3
            1 2 0
            2 3 0
            1 -3 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    fn should_ignore_comments() {
        let tmpfile = create_tempfile!("
            c comment
            p cnf 3 3
            c comment
            1 2 0
            2 3 0
            c comment
            1 -3 0
            c comment
            c comment
            c comment
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "duplicate problem")]
    fn duplicate_problem_statement() {
        let tmpfile = create_tempfile!("
            p cnf 3 3
            1 2 0
            p cnf 3 3
            2 3 0
            1 -3 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    #[should_panic(expected = "duplicate problem")]
    fn duplicate_problem_statement_2() {
        let tmpfile = create_tempfile!("
            p cnf 3 3
            p cnf 3 3
            1 2 0
            2 3 0
            1 -3 0
        ");
        let _ = parse_dimacs_file(tmpfile).unwrap();
    }

    #[test]
    fn adds_unit_clauses() {
        let tmpfile = create_tempfile!("
            p cnf 3 4
            1 0
            2 0
            3 0
            -1 -3 0
        ");
        let cnf = parse_dimacs_file(tmpfile).unwrap();
        assert_eq!(cnf.units.len(), 3);
    }

    // TODO: Make this doc test (with unit propagate); create_tempfile may need to be an importable
    // macro up there
    #[test]
    fn unit_propagate_works() {
        let tmpfile = create_tempfile!("
            p cnf 2 2
            1 0
            1 2 0
        ");
        let mut cnf = parse_dimacs_file(tmpfile).unwrap();
        cnf.unit_propagate(0);
        assert!(cnf.units.is_empty());
    }

    #[test]
    fn unit_propagate_works_2() {
        let tmpfile = create_tempfile!("
            p cnf 4 4
            1 0
            1 2 0
            2 3 0
            4 0
        ");
        let mut cnf = parse_dimacs_file(tmpfile).unwrap();
        cnf.unit_propagate(0);
        // Only one unit clause left
        assert_eq!(cnf.units.len(), 1);
        cnf.unit_propagate(3);
        // No more unit clauses
        assert!(cnf.units.is_empty());
    }

    /// This one is making sure that if unit_propagate removes a separate unit clause, the separate
    /// unit clause can still be unit propagated and will fail gracefully
    #[test]
    fn unit_propagate_works_special() {
        let tmpfile = create_tempfile!("
            p cnf 2 6
            1 0
            1 0
            2 0
            2 0
            2 0
            2 0
        ");
        let mut cnf = parse_dimacs_file(tmpfile).unwrap();
        // There are six clauses and all of them are units. One iteration of dpll will attempt to
        // remove all of these clauses. Even though unit_propagate(0) and unit_propagate(2) will
        // remove all of the clauses, the other unit_propagates should not panic.
        cnf.unit_propagate(0);
        cnf.unit_propagate(1);
        cnf.unit_propagate(2);
        cnf.unit_propagate(3);
        cnf.unit_propagate(4);
        cnf.unit_propagate(5);
        // The result should be an empty cnf - no clauses, no unit clauses.
        assert_eq!(cnf.clauses.len(), 0);
        assert_eq!(cnf.units.len(), 0);
    }

    #[test]
    fn remove_negation_adds_units() {
        let tmpfile = create_tempfile!("
            p cnf 4 4
            1 0
            2 -1 0
            3 -1 0
            4 1 0
        ");
        // After propagating unit clause 0, there should be two unit clauses in the subsequent
        // formula.
        let mut cnf = parse_dimacs_file(tmpfile).unwrap();
        cnf.unit_propagate(0);
        assert_eq!(cnf.units.len(), 2);
        // Unit prop should have removed clauses 0 and 3
        assert!(!cnf.units.contains(&0));
        assert!(!cnf.units.contains(&3));

        // But should have added clauses 2 and 3
        assert!(cnf.units.contains(&1));
        assert!(cnf.units.contains(&2));
    }

    #[test]
    fn propagation_tracks_nclause() {
        let tmpfile = create_tempfile!("
            p cnf 4 4
            1 0
            2 -1 0
            3 -1 0
            4 1 0
        ");
        let mut cnf = parse_dimacs_file(tmpfile).unwrap();
        cnf.unit_propagate(0);
        assert_eq!(cnf.nclause, 2);
        cnf.propagate(4); // 2
        assert_eq!(cnf.nclause, 1);
        cnf.propagate(6); // 3
        assert_eq!(cnf.nclause, 0);
    }

    #[test]
    fn zeroth_works() {
        let mut hs = HashSet::new();
        hs.insert(5);
        assert_eq!(zeroth!(hs), 5);
    }

    #[test]
    fn zeroth_works_2() {
        let mut hs = HashSet::new();
        hs.insert(5);
        hs.insert(535);
        let zth = zeroth!(hs);
        assert!(zth == 5 || zth == 535);
    }
}
