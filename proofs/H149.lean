import Mathlib
namespace H149
theorem fraction_bound (r : ℝ) (hr : 0 ≤ r) (hR : r ≤ 1) :
    0 ≤ (12/5)*r ∧ (12/5)*r ≤ 12/5 := by
  constructor <;> linarith
end H149
