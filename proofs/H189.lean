import Mathlib
namespace H189
theorem pulse_cost : (3/64:ℚ)^2=9/4096 := by norm_num
theorem no_common_reading (a b y eps : ℚ)
 (hsep : 2*eps < |a-b|)
 (ha : |a-y| ≤ eps) (hb : |y-b| ≤ eps) : False := by
 have htri : |a-b| ≤ |a-y|+|y-b| := abs_sub_le a y b
 linarith
theorem probe_nonnegative (n : ℕ) : (0:ℚ) ≤ n*(9/4096:ℚ) := by positivity
end H189
