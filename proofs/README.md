# Circle capacity proof

Run `make prove-circle` from the repository root. Docker Compose builds the
pinned Lean and Aeneas toolchain and checks the proof inside the container.

Charon reads the Rust source of the same capacity predicate used by
`Circle::validate_capacity`. Aeneas translates it to Lean, and
`CircleCapacitySpec.lean` checks it against the integer specification
`3 ≤ capacity` for every Rust `i16`.

The proof also uses the Rust capacity transition called by
`Circle::apply_event`. It checks that omitting capacity keeps the current
value, and that replaying any number of updates preserves the lower bound
when the creation capacity and all explicitly supplied capacities are at
least 3.

The replay theorem models only the capacity field of one creation event
followed by update events. Lean proves the loop in that model using the
generated Rust transition. Event ordering, history validation, and other
fields of `Circle::replay` are outside the theorem.
