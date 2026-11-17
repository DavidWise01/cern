import Mathlib
namespace H084
theorem band_energy (a b c : ℝ) (h : a+b+c=0) :
    4*((a-b)^2+(b-c)^2+(c-a)^2) = 12*(a^2+b^2+c^2) := by
  have hz : (a+b+c)^2 = 0 := by rw [h]; norm_num
  nlinarith [hz]
end H084
