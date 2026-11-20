import Mathlib
namespace H147
theorem budget_bound (a b c d m : ℝ) (h : a+b+c+d=4)
    (ha : m≤a) (hb : m≤b) (hc : m≤c) (hd : m≤d) :
    2*m≤2 := by linarith
theorem equal_optimum (a b c d : ℝ) (h : a+b+c+d=4)
    (ha : 1≤a) (hb : 1≤b) (hc : 1≤c) (hd : 1≤d) :
    a=1 ∧ b=1 ∧ c=1 ∧ d=1 := by
  constructor
  · linarith
  constructor
  · linarith
  constructor <;> linarith
end H147
