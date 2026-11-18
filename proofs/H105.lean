import Mathlib
namespace H105
theorem ten_components : 4 + 6 = (10 : Nat) := by decide
theorem quadratic_cross (a b c d x y : ℝ) :
  a*x^2+b*x*y+b*y*x+d*y^2 = a*x^2+2*b*x*y+d*y^2 := by ring
theorem level_scale (q r s : ℝ) (hs : s ≠ 0) :
    s*q = s*r ↔ q = r := by
  constructor
  · intro h
    exact mul_left_cancel₀ hs h
  · intro h
    rw [h]
end H105
