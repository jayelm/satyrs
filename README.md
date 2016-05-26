# satyrs

![Satyr Logo](logo.png)

The sexiest SAT solver out there.

## Usage

- Download the ZIP or Clone this repository
- Supply a valid CNF file (DIMACS format)

Run `cargo build` with optional `--release` flag and use `cargo run` or the
following command:

    target/{debug,release}/satyrs <file.cnf>

Repository comes many test files of various lengths, poke around for details.

Currently, two heuristics are implemented: one-sided Jeroslow-Wang and random
selection. We don't currently have functionality to switch between these two
using the command-line; instead, just replace the call to `let lit =
jw(&_cnf)` in `dpll.rs` with `random`.

## Does it work?

Yes! At least, on the problems that we've tested. If you find something that
doesn't work, do let us know.

## Is it fast?

No! In fact, it's slow enough to be absolutely impractical except as an
educational introduction to DPLL and Rust (which this was). There are a couple
of times where, due to our lack of experience, were unable to figure out a fast
way of implementing certain operations that complied with Rust's borrow
checker, and thus settled for ugly and slow workarounds including unnecessary
copying.

Here's a table of benchmarks on some selected CNF problems. As you can see, JW is
highly more efficient than random selection, but the runtime of our algorithm
in general blows up very quickly to the point where we lost patience with
random selection on the last problem. For those interested, `timing.py` can
generate these statistics.

| CNF          | Variables | Clauses | Satisfiable? | Mean JW Runtime (sd) | Mean Rand Runtime (sd) |
|--------------|-----------|---------|--------------|----------------------|------------------------|
| test.cnf     | 3         | 3       | Yes          | 0.01 (0.02)          | 0.00 (0.00)            |
| cascade.cnf  | 4         | 4       | Yes          | 0.00 (0.00)          | 0.00 (0.00)            |
| quinn.cnf    | 16        | 18      | Yes          | 0.00 (0.00)          | 0.00 (0.00)            |
| medium.cnf   | 50        | 80      | Yes          | 1.40 (1.17)          | 3.07 (2.13)            |
| hole6.cnf    | 42        | 133     | No           | 0.12 (0.0)           | 0.94 (0.02)            |
| hole7.cnf    | 56        | 204     | No           | 1.16 (0.25)          | 6.86 (0.31)            |
| uf250-01.cnf | 250       | 1065    | Yes          | 10.82 (NA)           | DNF                    |
