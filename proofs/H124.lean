import Mathlib
namespace H124
theorem resistance_bound (a b m d eps delta s c : ℝ)
    (he : 0 ≤ eps) (hd : 0 ≤ delta)
    (ha : |a| ≤ eps) (hb : |b| ≤ eps)
    (hm : |m| ≤ eps) (hD : |d| ≤ delta)
    (hn : s^2+c^2=1) :
    |m+d-(s^2*a+c^2*b)| ≤ 2*eps+delta := by
  have h₁ := abs_le.mp ha
  have h₂ := abs_le.mp hb
  have h₃ := abs_le.mp hm
  have h₄ := abs_le.mp hD
  apply abs_le.mpr
  constructor <;> nlinarith [
    mul_nonneg (sq_nonneg s) (sub_nonneg.mpr h₁.2),
    mul_nonneg (sq_nonneg c) (sub_nonneg.mpr h₂.2),
    mul_nonneg (sq_nonneg s) (sub_nonneg.mpr h₁.1),
    mul_nonneg (sq_nonneg c) (sub_nonneg.mpr h₂.1)]
end H124
