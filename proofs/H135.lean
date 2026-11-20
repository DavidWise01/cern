import Mathlib
namespace H135
theorem signed_reserve (p q b d h e : ℝ)
    (hs : |p+b*q| ≤ h-e) (hd : |d| ≤ e) :
    |p+b*q+d| ≤ h := by
  calc
    |p+b*q+d| ≤ |p+b*q|+|d| := abs_add_le _ _
    _ ≤ h := by linarith
theorem convex_feedback (p q a b t r : ℝ)
    (ha : |p+a*q| ≤ r) (hb : |p+b*q| ≤ r)
    (ht : 0 ≤ t) (ht' : t ≤ 1) :
    |p+((1-t)*a+t*b)*q| ≤ r := by
  rw [show p+((1-t)*a+t*b)*q = (1-t)*(p+a*q)+t*(p+b*q) by ring]
  calc
    _ ≤ |(1-t)*(p+a*q)|+|t*(p+b*q)| := abs_add_le _ _
    _ = (1-t)*|p+a*q|+t*|p+b*q| := by
      rw [abs_mul, abs_mul, abs_of_nonneg (sub_nonneg.mpr ht'), abs_of_nonneg ht]
    _ ≤ r := by
      have h1 := mul_le_mul_of_nonneg_left ha (sub_nonneg.mpr ht')
      have h2 := mul_le_mul_of_nonneg_left hb ht
      nlinarith
end H135
