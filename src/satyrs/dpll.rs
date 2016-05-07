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
fn _dpll(_cnf: &CNF, mut p_assn: PartialAssignment) -> Option<PartialAssignment> {
    // If consistent set of literals, return True
    let cnf = _cnf.clone();
    if cnf.clauses.is_empty() {
        return Some(p_assn)  // Display optional value
    }
    for unit in &cnf.units {
		let clause = cnf.clauses.get(unit).expect("Unit clause not found!");
		println!("{:?}: IM JUST A LONELY UNIT CLAUSE", clause);
		cnf.propogate(clause[0]);
    }
    // If contains an empty clause return False
    // For every unit-clause, unit-propogate
    // For ever pure literal, pure literal assign
    // Choose literal L for split
    // Return DPLL with L and -L

    // TODO: Need to implement. Just testing with a PartialAssignment
    println!("PARTIAL ASSIGNMENT:\n{}", p_assn);
    for x in 0..(cnf.nvar) {
        p_assn.assign(x as usize, true);
    }
    Some(p_assn)
}
