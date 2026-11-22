import Mathlib
namespace H188
theorem residual_no_false_alarm (y prediction eps : ℚ)
 (h : |y-prediction| ≤ eps) : ¬ eps < |y-prediction| := not_lt.mpr h
theorem diagnostic_cost : (3/64:ℚ)^2 = 9/4096 := by norm_num
theorem nonnegative_stale_cost (pulses stale : ℕ) :
  (0:ℚ) ≤ pulses * (9/4096:ℚ) + stale * (1/128:ℚ) := by positivity
end H188
