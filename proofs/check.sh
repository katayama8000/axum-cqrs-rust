#!/bin/sh
set -eu

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$work"
cp /repo/proofs/CircleCapacitySpec.lean .

# Extract the same source file that Circle::validate_capacity calls.
charon rustc --preset=aeneas --sysroot default --dest-file=circle_capacity.llbc -- \
    /repo/src/crates/domain/src/aggregate/circle/capacity.rs \
    --crate-name=circle_capacity --crate-type=lib
aeneas -backend lean -namespace CircleCapacity -warnings-as-errors -no-progress-bar \
    -dest "$work" circle_capacity.llbc

cd /opt/aeneas/backends/lean
lake env sh -ec '
    cd "$1"
    export LEAN_PATH="$PWD:$LEAN_PATH"
    lean -o CircleCapacity.olean CircleCapacity.lean
    lean -DwarningAsError=true CircleCapacitySpec.lean
' sh "$work"

printf '%s\n' 'Circle capacity proof passed.'
