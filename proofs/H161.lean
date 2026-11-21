import Mathlib
namespace H161
theorem three_signers_one_information_root :
 (0 : Nat) ≠ 1 ∧ 1 ≠ 2 ∧ 0 ≠ 2 ∧ (0 : Nat) = 0 := by decide
theorem independent_roots (a b c : Nat)
 (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) :
 a ≠ b ∧ a ≠ c ∧ b ≠ c := ⟨hab,hac,hbc⟩
end H161
