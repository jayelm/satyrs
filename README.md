# satyrs

![Satyr Logo](logo.png)

The sexiest SAT solver out there.

## Potential optimizations

- Use sets for clauses instead of vectors. Faster removal, slower clone?
- Units attribute contains variables instead of clauses
- Have `propagate` and `unit_propagate` return false if the clause is no longer
    satisfiable, prevents requiring check for empty unit clauses at beginning
