import Mathlib
namespace H111
-- Algebraic endpoint enclosure, conditional on monotone segment behavior.
theorem endpoint_enclosure (x y z r : ℝ)
    (hx : x ≤ r) (hy : y ≤ r)
    (hz : min x y ≤ z ∧ z ≤ max x y) : z ≤ r :=
  le_trans hz.2 (max_le hx hy)
theorem convex_prefix (x y r a : ℝ) (hx : x ≤ r) (hy : y ≤ r)
    (ha : 0 ≤ a) (hb : a ≤ 1) :
    a*x+(1-a)*y ≤ r := by
  have h1 := mul_le_mul_of_nonneg_left hx ha
  have h2 := mul_le_mul_of_nonneg_left hy (sub_nonneg.mpr hb)
  linarith [show a*r+(1-a)*r=r by ring]
end H111
