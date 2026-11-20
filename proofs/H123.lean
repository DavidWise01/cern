import Mathlib
namespace H123
theorem rotation_recovery (c s a b q m : ℝ)
    (h : 2*s*c ≠ 0)
    (hm : m = s^2*a + 2*s*c*q + c^2*b) :
    q = (m-s^2*a-c^2*b)/(2*s*c) := by
  apply (eq_div_iff h).2
  linarith
end H123
