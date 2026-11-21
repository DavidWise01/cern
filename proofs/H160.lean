import Mathlib
namespace H160
-- These lemmas formalize arithmetic necessary conditions. Signed
-- provenance and real-world independence require external assumptions.
theorem three_independent_sources (a b c : Nat)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) :
    a ≠ b ∧ a ≠ c ∧ b ≠ c := ⟨hab,hac,hbc⟩
theorem recovered_gap : (4 / 5 : ℚ) = 2 * (2 / 5 : ℚ) := by norm_num
end H160
