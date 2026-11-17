import Mathlib
namespace H088
-- Conditional algebraic final step, not the full graph inequality.
theorem aggregate_bound (F E M : ℝ) (hF : 0 ≤ F)
    (h : F * M ≤ (600 + F / 4) * E) :
    F / (600 + F / 4) * M ≤ E := by
  have hp : 0 < 600 + F / 4 := by linarith
  rw [div_mul_eq_mul_div]
  exact (div_le_iff₀ hp).2 (by linarith)
end H088
