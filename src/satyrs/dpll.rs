use satyrs::cnf::{CNF, Assignment, PartialAssignment};

/**
 * DPLL Algorithm implementation
 * Start by cloning the cnf so we can modify the HashMap
 * Clone might be slow, consider a faster option
 * TODO: Consider using HashSet instead of Vec for the clauses
 */
#[allow(non_snake_case)]
pub fn DPLL(cnf: &CNF) -> Option<Assignment> {
    //let _cnf = cnf.clone();
    //println!("DPLL {:?}", _cnf);
    let p_assn = PartialAssignment::new(cnf.nvar as usize);
    match _dpll(cnf, p_assn){
        Some(assn) => Some(assn.assignment.iter().map(|a| a.unwrap()).collect()),
        None => None
    }
}

#[allow(unused_variables)]
fn _dpll(cnf: &CNF, mut p_assn: PartialAssignment) -> Option<PartialAssignment> {
    // If consistent set of literals, return True
    let mut _cnf = cnf.clone();
    if cnf.clauses.is_empty() {
        return Some(p_assn);  // Display optional value
    }

    // If contains an empty clause return None
    // TODO: Consider optimizing
    for clause in cnf.clauses.values() {
        if clause.len() == 0 {
            return None;
        }
    }

    // For every unit-clause, unit-propogate
    for unit in cnf.units.iter() {
        let clause = cnf.clauses.get(&unit).expect("Clause not found");
        let lit = clause.iter().next().unwrap();
        p_assn.assign_literal(lit);
        _cnf.unit_propagate(*unit); // Only propogate in the clone
    }

    // TODO: For every pure literal, pure literal assign
    // Choose literal L for split
    let literal: i32 = cnf.clauses.values().next().unwrap().iter().next().unwrap();

    // Return DPLL with L and -L
    p_assn.assign_literal(literal);
    let left = _dpll(&_cnf, p_assn);
    left
    // TODO: Copy p_assn. Right now rust is complaining about borrowing.
    // That will let us include the second part of this algorithm, below:
    // if left.is_some() { return left; }
    // p_assn.unassign_literal(literal);
    // p_assn.assign_literal(literal ^ 1);
    // _dpll(&_cnf, p_assn)
}
