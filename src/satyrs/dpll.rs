use satyrs::cnf::CNF;

/**
 * DPLL Algorithm implementation
 * Start by cloning the cnf so we can modify the HashMap
 * Clone might be slow, consider a faster option
 */
#[allow(non_snake_case)]
pub fn DPLL(cnf: &CNF) -> bool {
    let cnf_clone = cnf.clone();
    println!("{:?}",cnf_clone);
    true
}
