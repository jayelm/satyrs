use satyrs::cnf::CNF;



/**
 * DPLL Algorithm implementation
 * Start by cloning the cnf so we can modify the HashMap
 * Clone might be slow, consider a faster option
 */
#[allow(non_snake_case)]
pub fn DPLL(cnf: &CNF) -> bool {
    let cnf_clone = cnf.clone();
    // If consistent set of literals, return True
    // If contains an empty clause return False
    // For every unit-clause, unit-propogate
    // For ever pure literal, pure literal assign
    // Choose literal L for split
    // Return DPLL with L and -L
    println!("{:?}",cnf_clone);
    true
}
