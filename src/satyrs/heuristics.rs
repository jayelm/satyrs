//! Heuristics for CNF instances.
use satyrs::cnf::CNF;

// K selected heuristically for MOM formula
#[allow(dead_code)]
const MOM_K: i32 = 10;

/// Chooses the literal with the Maximum Occurrence of clauses of minimum size.
/// Specifically, let $f*(x)$ be the number of unresolved smallest clauses containing $x$.
/// Choose $x$ that maximizes $((f&(x)) + f*(-x)) * 2^k + f*(x) * f*(!x)$.
///
/// This method gives preference to selecting literals (and negations) that occur frequently in
/// small clauses.
#[allow(dead_code, unused_variables)]
pub fn mom(cnf: &CNF) {
    unimplemented!();
}

/// One-sided jeroslow-wang heuristic. Counts the number of clauses a literal appears in,
/// weighting smaller clauses more heavily, with the formula
/// $$J(l) = \sum_{\{\omega \in \phi \mid l \in \omega}\} 2^{-|\omega|}.$$
/// In practice, one-sided is faster than two-sided, and this method can be ~30x faster than MOM!
#[allow(dead_code)]
pub fn jw(cnf: &CNF) -> i32 {
    // Can't use max_by because f64 doesn't implement total Ord. Until this works, we'll do it the
    // for loop way.
    let j = |lit_ptr: &i32| -> f64 {
        cnf.occurrences
           .get(lit_ptr)
           .unwrap()
           .iter()
           .fold(0f64,
                 |acc, occ| acc + (2f64).powi(-(cnf.clauses.get(occ).unwrap().len() as i32)))
    };
    // (Alternative functional one-liner for the mess below)
    // *cnf.occurrences.keys().max_by_key(j).unwrap()
    let mut max_j: f64 = 0_f64;
    let mut max_lit: i32 = -1;
    for lit in cnf.occurrences.keys() {
        let lit_j = j(lit);
        if lit_j > max_j {
            max_j = lit_j;
			max_lit = *lit;
		}
	}
	if max_lit == -1 {
		panic!("Called heuristic on formula with no occurrences");
	}
	max_lit
}

#[allow(dead_code)]
pub fn random(cnf: &CNF) -> i32 {
	let literal = match cnf.clauses.values().next() {
		Some(clause) => {
			if !clause.is_empty() {
				Some(zeroth!(clause))
			} else {
				None
			}
		}
		None => None
	};
	if literal.is_none() { panic!("No literals in clause!"); }
	literal.unwrap()
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::fs::File;
    use std::io::SeekFrom;
    use satyrs::cnf::parse_dimacs_file;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn jw_works() {
        let tmpfile = create_tempfile!("
            p cnf 4 4
            1 2 0
            1 3 0
            1 4 0
            -1 2 0
        ");
        let cnf = parse_dimacs_file(tmpfile).unwrap();
        // The heuristic should select 1 as the variable
        // (2 as the literal)
        assert_eq!(jw(&cnf), 2);
    }

    #[test]
    fn jw_works_2() {
        let tmpfile = create_tempfile!("
            p cnf 5 5
            1 2 0
            1 3 5 0
            2 5 0
            1 4 3 0
            -1 2 0
        ");
        // Now, although 1 occurs several times, 2 occurs the most in unit clauses
        let cnf = parse_dimacs_file(tmpfile).unwrap();
        assert_eq!(jw(&cnf), 4); // I.e.
    }
}
