import Mathlib
namespace H087
-- Algebraic Cauchy-Schwarz step, not the full graph theorem.
theorem path_face_bound (a b c d : ℝ) :
    (a*c+b*d)^2 ≤ (a^2+b^2)*(c^2+d^2) := by
  nlinarith [sq_nonneg (a*d-b*c)]
theorem conditional_gap (delta energy mass : ℝ)
    (h : delta^2 * mass ≤ (72+30*delta^2)*energy) :
    delta^2 / (72+30*delta^2) * mass ≤ energy := by
  have hp : 0 < 72+30*delta^2 := by positivity
  rw [div_mul_eq_mul_div]
  exact (div_le_iff₀ hp).2 (by linarith)
end H087
