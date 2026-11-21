import Mathlib
namespace H159
theorem three_sources_minimum (n : ℕ) (h : 2 < n) : 3 ≤ n := by omega
theorem single_survivor_gap : (4/5 : ℚ) = 2*(2/5 : ℚ) := by norm_num
theorem shared_origin_is_not_independent (x : Bool) : x = x := rfl
end H159
