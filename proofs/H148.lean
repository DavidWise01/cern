import Mathlib
namespace H148
theorem robust_upper (a b g : ℝ) (budget : 4*a+6*b=4)
    (ha : g≤6*a) (hb : g≤6*b) : g≤12/5 := by linarith
end H148
