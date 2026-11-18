import Mathlib
namespace H090
-- Supporting scalar budget inequality; not the full holonomy theorem.
theorem budget_energy (a b c e : ℝ)
    (ha : 0 ≤ a) (hb : 0 ≤ b) (hc : 0 ≤ c)
    (he : 0 ≤ e) (h : a + b + c ≤ 1) :
    a*e + b*e + c*e ≤ e := by
  calc
    a*e + b*e + c*e = (a+b+c)*e := by ring
    _ ≤ 1*e := mul_le_mul_of_nonneg_right h he
    _ = e := one_mul e
end H090
