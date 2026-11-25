import CircleCapacity

open Aeneas.Std

/-- The result of the Rust predicate agrees with the integer specification. -/
theorem is_valid_spec (capacity : I16) :
    CircleCapacity.is_valid capacity = .ok (decide (3 ≤ capacity.val)) := by
  rfl

/-- The Rust transition keeps the previous capacity when the update omits it. -/
theorem apply_update_spec (current : I16) (proposed : Option I16) :
    CircleCapacity.apply_update current proposed = .ok (proposed.getD current) := by
  cases proposed <;> rfl

/-- Replay the capacity fields of update events in order after the creation event. -/
def replayCapacity (current : I16) : List (Option I16) → Result I16
  | [] => .ok current
  | proposed :: rest =>
    Aeneas.Std.bind (CircleCapacity.apply_update current proposed) (fun next =>
      replayCapacity next rest)

def ValidCapacityUpdate : Option I16 → Prop
  | none => True
  | some capacity => 3 ≤ capacity.val

/-- Valid updates preserve the capacity bound for a history of any length. -/
theorem replay_capacity_preserves_minimum
    (initial : I16) (updates : List (Option I16))
    (initialValid : 3 ≤ initial.val)
    (updatesValid : ∀ proposed ∈ updates, ValidCapacityUpdate proposed) :
    ∃ final, replayCapacity initial updates = .ok final ∧ 3 ≤ final.val := by
  induction updates generalizing initial with
  | nil =>
    exact ⟨initial, rfl, initialValid⟩
  | cons proposed rest ih =>
    have proposedValid := updatesValid proposed (List.mem_cons_self)
    have nextValid : 3 ≤ (proposed.getD initial).val := by
      cases proposed with
      | none => exact initialValid
      | some _ => exact proposedValid
    have restValid : ∀ update ∈ rest, ValidCapacityUpdate update :=
      fun update member => updatesValid update (List.mem_cons_of_mem _ member)
    obtain ⟨final, replayed, finalValid⟩ := ih _ nextValid restValid
    refine ⟨final, ?_, finalValid⟩
    rw [replayCapacity, apply_update_spec, bind_ok]
    exact replayed
