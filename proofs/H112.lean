import Mathlib
namespace H112
theorem pause_safe (x r a : ℝ) (hx : 0 ≤ x) (hr : x ≤ r)
    (ha : a ≤ 1) : a*x ≤ r := by
  exact le_trans (by simpa using mul_le_mul_of_nonneg_right ha hx) hr

theorem reduced_forcing (x a b f g : ℝ) (hb : 0 ≤ b)
    (h : f ≤ g) : a*x+b*f ≤ a*x+b*g := by
  exact add_le_add_left (mul_le_mul_of_nonneg_left h hb) _
end H112
