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
    let mut p_assn = PartialAssignment::new(cnf.nvar as usize);
    match _dpll(cnf, &mut p_assn) {
        Some(assn) => Some(assn.assignment.iter().map(|a| { 
            println!("WE HERE ");
            match *a {
                Some(a) => a,
                None => true,
            }
        }).collect()),
        None => None,
    }
}

#[allow(unused_variables)]
fn _dpll(cnf: &CNF, p_assn: &mut PartialAssignment) -> Option<PartialAssignment> {
    // If consistent set of literals, return True
    println!("{}\n{}",cnf,p_assn);
    if cnf.clauses.is_empty() {
        let _p_assn = p_assn.clone();
        return Some(_p_assn);  // Display optional value
    }

    // If contains an empty clause return None
    // TODO: Consider optimizing
    for clause in cnf.clauses.values() {
        if clause.len() == 0 {
            return None;
        }
    }

    // For every unit-clause, unit-propogate
    let mut _cnf = cnf.clone();
    for unit in cnf.units.iter() {
        let clause = cnf.clauses.get(&unit).expect("Clause not found");
        let lit : i32 = zeroth!(clause);
        p_assn.assign_literal(lit);
        _cnf.unit_propagate(*unit); // Only propogate in the clone
    } // FIXME: Unit propogate might be the end of things
    println!("After Prop! {}",_cnf);
    // Clone for the right
    let mut r_cnf = _cnf.clone();
    // TODO: For every pure literal, pure literal assign
    // Choose literal L for split
    //let literal: i32 = zeroth!(_cnf.clauses.values().next().unwrap());
    let literal = match _cnf.clauses.values().next() {
        Some(thing) => Some(zeroth!(thing)),
        None => None,
    };
    
    if literal.is_none() {
        let _p_assn = p_assn.clone();
        return Some(_p_assn);
    }
    let lit = literal.unwrap();
    println!("Split on {} ({})",lit,lit/2);
    // Propagate literal
    _cnf.propagate(lit);

    // Return DPLL with L and -L
    p_assn.assign_literal(lit);
    let left = _dpll(&_cnf, p_assn);
    if left.is_some() { return left; }
    p_assn.unassign_literal(lit);
    p_assn.assign_literal(lit ^ 1);
    r_cnf.propagate(lit^1);
    _dpll(&_cnf, p_assn)
}
