import CircleCapacity

open Aeneas.Std

/-- The result of the Rust predicate agrees with the integer specification. -/
theorem is_valid_spec (capacity : I16) :
    CircleCapacity.is_valid capacity = .ok (decide (3 ≤ capacity.val)) := by
  rfl
