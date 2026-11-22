import Mathlib
namespace H184
theorem no_shared_bounded_observation (x y eps obs : ℚ)
 (hsep : 2 * eps < |x - y|)
 (hx : |obs-x| ≤ eps) (hy : |obs-y| ≤ eps)
 (heps : 0 ≤ eps) : False := by
 have htri := abs_sub_le x obs y
 have h1 : |x-y| ≤ |x-obs|+|obs-y| := by
   simpa [abs_sub_comm] using htri
 have h2 : |x-obs| ≤ eps := by simpa [abs_sub_comm] using hx
 linarith
theorem pulse_cost_grid : (3/64:ℚ)^2 = (9/4096:ℚ) := by norm_num
theorem strict_threshold : (1/32:ℚ) < 3/64 := by norm_num
end H184
