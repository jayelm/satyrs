//! DPLL Algorithm implementation with unit-clause propagation and
//! (TODO) pure literal assignment.

use satyrs::cnf::{CNF, Assignment, PartialAssignment};

#[allow(non_snake_case)]
pub fn DPLL(cnf: &CNF, verbose: bool) -> Option<Assignment> {
    let mut p_assn = PartialAssignment::new(cnf.nvar as usize);
    match _dpll(cnf, &mut p_assn, verbose) {
        Some(assn) => Some(assn.assignment.iter().map(|a| {
            match *a {
                Some(a) => a,
                None => true
            }
        }).collect()),
        None => None
    }
}

fn _dpll(cnf: &CNF, p_assn: &mut PartialAssignment, verbose: bool) -> Option<PartialAssignment> {
    if verbose {
        println!("====DPLL====\n");
        println!("{}\n{}", cnf, p_assn);
    }
    // If consistent set of literals, return True
    if cnf.clauses.is_empty() {
        let _p_assn = p_assn.clone();
        return Some(_p_assn);  // Display optional value
    }

    // If contains an empty clause return None
    for clause in cnf.clauses.values() {
        if clause.is_empty() {
            return None;
        }
    }

    // For every unit-clause, unit-propogate
    let mut _cnf = cnf.clone();
    for unit in cnf.units.iter() {
        // Previous versions of this for loop could have unit-propagated and
        // remove the unit clause here, so it's not necessary that the unit clause id
        // still exists in the clauses.
        if let Some(clause) = cnf.clauses.get(&unit) {
            // Then via unit propagation we've created an empty clause; no solution down this path
            if clause.is_empty() {
                return None;
            }
            let lit : i32 = zeroth!(clause);
            p_assn.assign_literal(lit);
            _cnf.unit_propagate(*unit); // Only propogate in the clone
            // TODO: Could have check for empty clauses in unit_propagate
        }
    }
    // Clone for the right literal, since we're going to propagate _cnf with the left literal
    let mut r_cnf = _cnf.clone();

    // TODO: For every pure literal, pure literal assign
    // Choose literal L for split
    let literal = match _cnf.clauses.values().next() {
        Some(clause) => {
            if !clause.is_empty() {
                Some(zeroth!(clause))
            } else {
                // This will return out of the *function* - we found an empty clause, so formula is
                // not satisfiable.
                return None;
            }
        }
        None => None,
    };

    if literal.is_none() {
        let _p_assn = p_assn.clone();
        return Some(_p_assn);
    }
    let lit = literal.unwrap();
    if verbose {
        if lit & 1 == 0 { // True
            println!("Splitting on {}", lit / 2);
        } else { // False
            println!("Splitting on -{}", lit / 2);
        }
    }
    // Propagate literal
    _cnf.propagate(lit);

    // Return DPLL with L and -L
    p_assn.assign_literal(lit);

    if verbose { println!("Trying left"); }
    let left = _dpll(&_cnf, p_assn, verbose);
    // If this branch works, return left.
    if left.is_some() { return left; }

    // Otherwise, unassign this literal, assign its negation, and try the subsequent formula.
    p_assn.unassign_literal(lit);
    p_assn.assign_literal(lit ^ 1);

    r_cnf.propagate(lit ^ 1);
    if verbose { println!("Trying right"); }
    _dpll(&r_cnf, p_assn, verbose)
}
