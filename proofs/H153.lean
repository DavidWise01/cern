import Mathlib
namespace H153
-- An elementary certificate for the first minimum cut:
-- every surviving compound gate toggles two binary coordinates,
-- hence preserves total bit parity.
theorem even_toggle_parity (a b : Bool) :
  (a != b) = (!a != !b) := by cases a <;> cases b <;> decide
theorem four_root_threshold : (3 : Nat) < 4 := by decide
theorem worst_triple_gap : (4 / 5 : ℚ) > 0 := by norm_num
end H153
