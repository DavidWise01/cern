import Mathlib
namespace H108
-- Algebraic form of the prefix reserve condition.
theorem reserve_gate (g x r exposure : ℝ) (hg : 0 < g)
    (h : x + exposure ≤ r*g) :
    (x+exposure)/g ≤ r := by
  exact (div_le_iff₀ hg).2 h
theorem segment_convex (x equilibrium r a : ℝ)
    (hx : x ≤ r) (he : equilibrium ≤ r)
    (ha : 0 ≤ a) (ha' : a ≤ 1) :
    a*x + (1-a)*equilibrium ≤ r := by
  have h1 := mul_le_mul_of_nonneg_left hx ha
  have h2 := mul_le_mul_of_nonneg_left he (sub_nonneg.mpr ha')
  linarith [show a*r+(1-a)*r=r by ring]
end H108
