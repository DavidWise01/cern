import Mathlib
namespace H122
-- Scalar form of the obstruction's quadratic evaluation.
theorem hidden_product (a b : ℝ) (h : a = 0 ∨ b = 0) :
    2*a*b = 0 := by
  rcases h with h | h <;> simp [h]

theorem mixed_identifies (base coupling x y : ℝ)
    (hc : coupling ≠ 0)
    (h : base + coupling*x = base + coupling*y) : x = y := by
  have hm : coupling*(x-y) = 0 := by nlinarith
  rcases mul_eq_zero.mp hm with hz | hz
  · exact (hc hz).elim
  · linarith
end H122
