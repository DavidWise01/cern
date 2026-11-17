import Mathlib
namespace H085
theorem transport_nonnegative {E : Type*} [Fintype E]
    (r : E → ℝ) : 0 ≤ ∑ e, (r e)^2 := by
  exact Finset.sum_nonneg (fun e _ => sq_nonneg (r e))

theorem covariant_link {G : Type*} [Group G]
    (a b u v : G) : (a * u * b⁻¹) * (b * v) = a * (u * v) := by
  group
end H085
