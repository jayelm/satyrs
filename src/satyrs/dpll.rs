//! DPLL Algorithm implementation with unit-clause propagation and
//! pure literal assignment.

use satyrs::cnf::{CNF, Assignment, PartialAssignment};
use satyrs::heuristics::jw;

#[allow(non_snake_case)]
pub fn DPLL(cnf: &CNF, verbose: bool) -> Option<(Assignment,PartialAssignment)> {
    let mut p_assn = PartialAssignment::new(cnf.nvar as usize);
    match _dpll(cnf, &mut p_assn, verbose) {
        Some(assn) => {
            Some((assn.assignment
                     .iter()
                     .map(|a| {
                         match *a {
                             Some(a) => a,
                             None => true,
                         }
                     })
                     .collect(),
                     assn))
        }
        None => None,
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

    // For every unit-clause, unit-propagate
    let mut _cnf = cnf.clone();
    let mut units = _cnf.units.clone();
    while !units.is_empty() {
        if verbose { println!("Simplifying unit clause(s): {:?}", units) }
        for unit in units.iter() {
            // Previous versions of this for loop could have unit-propagated and
            // remove the unit clause here, so it's not necessary that the unit clause id
            // still exists in the clauses.
            if let Some(clause) = _cnf.clauses.get(&unit) {
                // Then via unit propagation we've created an empty clause; no solution down this path
                if clause.is_empty() {
                    return None;
                }
                let lit: i32 = zeroth!(clause);
                p_assn.assign_literal(lit);
                // TODO: Could have check for empty clauses in unit_propagate
            }
            _cnf.unit_propagate(*unit); // Only propagate in the clone
        }    
        units = _cnf.units.clone();
    }
    // Pure literal elimination
    // We can iterate through old cnf occurrences and propagate on new _cnf
    for lit in cnf.occurrences.keys() {
        let neg = *lit ^ 1;
        if !cnf.occurrences.contains_key(&neg) {
            p_assn.assign_literal(*lit);
            _cnf.propagate(*lit);
        }
    }

    // Clone for the right literal, since we're going to propagate _cnf with the left literal
    let mut r_cnf = _cnf.clone();

    // Choose literal L for split
    // Heuristics don't do random checks for empty occurrences.
    // If there are no occurrences,
    if _cnf.occurrences.is_empty() {
        // FIXME: horrible hack
        // Bookkeeping is not implemented perfectly, we can get to this state and (just from
        // empirics) clauses will either be EMPTY or will have any number of empty clauses.
        // Intutition: if clauses is EMPTY, then there are no occurrences but also no clauses, so
        // we're good to go. If clauses is NOT empty, then there are no occurrences but there are
        // clauses, so the clauses are empty, so we return none.
        if _cnf.clauses.is_empty() {
            // Good
            let _p_assn = p_assn.clone();
            return Some(_p_assn);
        } else {
            // Bad
            return None;
        }
    }
    let lit = jw(&_cnf);

    if verbose {
        if lit & 1 == 0 {
            // True
            println!("Splitting on {}", lit / 2);
        } else {
            // False
            println!("Splitting on -{}", lit / 2);
        }
    }
    // Propagate literal
    _cnf.propagate(lit);

    // Return DPLL with L and -L
    p_assn.assign_literal(lit);

    if verbose {
        println!("Trying left");
    }
    let left = _dpll(&_cnf, p_assn, verbose);
    // If this branch works, return left.
    if left.is_some() {
        return left;
    }

    // Otherwise, unassign this literal, assign its negation, and try the subsequent formula.
    let neg = lit ^ 1;
    p_assn.unassign_literal(lit);
    p_assn.assign_literal(neg);

    r_cnf.propagate(neg);
    if verbose {
        println!("Trying right");
    }
    _dpll(&r_cnf, p_assn, verbose)
}
