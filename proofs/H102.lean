import Mathlib
namespace H102
theorem positive_gap (k d : ℝ) (hk : 0 < k) (hd : 0 ≤ d) :
    0 < min (577*k) (k+d) := by
  exact lt_min (by positivity) (by linarith)
theorem failed_axes (k : ℝ) (hk : 0 ≤ k) :
    min (577*k) (k+0) = k := by
  rw [add_zero, min_eq_right (by linarith)]
theorem contrast_threshold (k d : ℝ) :
    577*k ≤ k+d ↔ 576*k ≤ d := by
  constructor <;> intro h <;> linarith
end H102
