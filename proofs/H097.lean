import Mathlib
namespace H097
theorem alternating_balance : (-1 : ℤ)+1-1+1-1+1=0 := by norm_num
theorem polarity_mode (k : ℝ) :
    (2+k)*(-1)-1-1 = (4+k)*(-1) ∧
    (2+k)*1-(-1)-(-1) = (4+k)*1 := by
  constructor <;> ring
theorem crossing (k : ℝ) : 7*k ≤ 1+k ↔ k ≤ 1/6 := by
  constructor <;> intro h <;> linarith
end H097
