import Mathlib
namespace H089
theorem convex_energy (t a b e : ℝ)
    (ht : 0 ≤ t) (ht' : t ≤ 1)
    (ha : a ≤ e) (hb : b ≤ e) :
    t*a+(1-t)*b ≤ e := by
  have h₁ := mul_le_mul_of_nonneg_left ha ht
  have h₂ := mul_le_mul_of_nonneg_left hb (sub_nonneg.mpr ht')
  linarith [show t*e+(1-t)*e=e by ring]
end H089
