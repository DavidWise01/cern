import Mathlib
namespace H181
-- Immediate-error nonincrease for a robust positive-interval action.
theorem positive_correct (theta u : ℚ)
  (hθ : 0 ≤ theta) (hu : 0 ≤ u) (hsmall : u ≤ theta) :
  |theta-u| ≤ |theta| := by
  rw [abs_of_nonneg (sub_nonneg.mpr hsmall), abs_of_nonneg hθ]
  linarith
theorem negative_correct (theta u : ℚ)
  (hθ : theta ≤ 0) (hu : u ≤ 0) (hlarge : theta ≤ u) :
  |theta-u| ≤ |theta| := by
  have h : theta-u ≤ 0 := by linarith
  rw [abs_of_nonpos h,abs_of_nonpos hθ]
  linarith
theorem unknown_delay_zero_correction (theta : ℚ) :
  |theta-(0:ℚ)|=|theta| := by ring
end H181
