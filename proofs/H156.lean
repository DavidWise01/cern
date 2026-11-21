import Mathlib
namespace H156
theorem weighted_bridge_eigenvalue (w : ℚ) : 2*w = w+w := by ring
theorem target_iff (w : ℚ) (hw : 0 ≤ w) :
    (4/5 : ℚ) ≤ 2*w ↔ (2/5 : ℚ) ≤ w := by constructor <;> intro h <;> linarith
theorem connectivity_not_full_recovery (w : ℚ) (hw : 0 < w)
    (hsmall : w < 2/5) : 0 < 2*w ∧ 2*w < 4/5 := by constructor <;> linarith
end H156
