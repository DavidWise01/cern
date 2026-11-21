import Mathlib
namespace H179
theorem bounded_upper (x c : ℚ) : min c (max (-c) x) ≤ c := min_le_left _ _
theorem bounded_lower (x c : ℚ) (hc : 0 ≤ c) :
    -c ≤ min c (max (-c) x) := by
  apply le_min
  · linarith
  · exact le_max_left _ _
theorem nonnegative_budget (delta : ℚ) : 0 ≤ |delta| := abs_nonneg _
theorem common_mode_invariance (a b drift : ℚ) :
    (a+drift)-(b+drift)=a-b := by ring
end H179
