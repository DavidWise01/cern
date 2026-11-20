import Mathlib
namespace H132
theorem reserve (b e : ℚ) (h : b+e ≤ 1/4) :
    3/4+b+e ≤ 1 := by linarith
theorem sharp (b e : ℚ) (h : 1/4 < b+e) :
    1 < 3/4+b+e := by linarith
end H132
