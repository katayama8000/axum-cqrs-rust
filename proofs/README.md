# Circle capacity proof

Run `make prove-circle` from the repository root. Docker Compose builds the
pinned Lean and Aeneas toolchain and checks the proof inside the container.

Charon reads the Rust source of the same capacity predicate used by
`Circle::validate_capacity`. Aeneas translates it to Lean, and
`CircleCapacitySpec.lean` checks it against the integer specification
`3 ≤ capacity` for every Rust `i16`.

The theorem covers the capacity predicate. Event replay applies recorded
values directly and is outside its scope.
