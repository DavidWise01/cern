import Mathlib
namespace H136
theorem robust_interval (l u y d h e : ℚ)
    (hl : -h+e ≤ l) (hu : u ≤ h-e)
    (hy1 : l ≤ y) (hy2 : y ≤ u)
    (hd : |d| ≤ e) : |y+d| ≤ h := by
  have hd' := abs_le.mp hd
  apply abs_le.mpr
  constructor <;> linarith [hd'.1, hd'.2]
theorem monotone_product (a l x u : ℚ)
    (ha : 0 ≤ a) (hl : l ≤ x) (hu : x ≤ u) :
    a*l ≤ a*x ∧ a*x ≤ a*u :=
  ⟨mul_le_mul_of_nonneg_left hl ha, mul_le_mul_of_nonneg_left hu ha⟩
end H136
