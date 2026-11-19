import Mathlib
namespace H116

theorem fractional_term (w c y z : ℚ)
    (hz : 0 ≤ z) (hz' : z ≤ 1) :
    w*z ≤ y*c*z + max 0 (w-y*c) := by
  have h₁ : w-y*c ≤ max 0 (w-y*c) := le_max_right _ _
  have h₂ := mul_le_mul_of_nonneg_right h₁ hz
  have h₃ := mul_le_mul_of_nonneg_left hz' (le_max_left (0 : ℚ) (w-y*c))
  nlinarith

theorem dual_capacity (y load cap : ℚ)
    (hy : 0 ≤ y) (hc : load ≤ cap) :
    y*load ≤ y*cap :=
  mul_le_mul_of_nonneg_left hc hy

theorem integral_prune (v incumbent : ℤ)
    (h : v < incumbent + 1) :
    v ≤ incumbent := by omega

end H116
