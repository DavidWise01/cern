import Mathlib
namespace H109
theorem admission (x r a b F : ℝ) (hb : 0 < b) :
    a*x+b*F ≤ r ↔ F ≤ (r-a*x)/b := by
  rw [le_div_iff₀ hb]
  constructor <;> intro h <;> linarith [mul_comm b F]
end H109
