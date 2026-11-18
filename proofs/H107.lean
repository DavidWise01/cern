import Mathlib
namespace H107
theorem boundary_gate (d r F : ℝ) (hr : 0 ≤ r) (hF : F ≤ d*r) :
    -2*d*r^2 + 2*r*F ≤ 0 := by
  have h := mul_le_mul_of_nonneg_left hF (show 0 ≤ 2*r by linarith)
  nlinarith
theorem threshold_sharp (d r F : ℝ) (hr : 0 < r) (hF : d*r < F) :
    0 < 2*r*(F-d*r) := by
  exact mul_pos (by linarith) (sub_pos.mpr hF)
end H107
