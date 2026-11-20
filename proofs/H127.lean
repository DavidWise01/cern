import Mathlib
namespace H127
theorem observed_radius (r d e : ℝ) :
    min (r+d) e ≤ e := min_le_right _ _
theorem propagated_radius (r d e : ℝ) :
    min (r+d) e ≤ r+d := min_le_left _ _
theorem bellman_choice (a b : ℝ) :
    min a b ≤ a ∧ min a b ≤ b :=
  ⟨min_le_left _ _, min_le_right _ _⟩
theorem greedy_better_when (a b c d : ℝ)
    (h : a ≤ b) (hfuture : c ≤ d) : a+c ≤ b+d :=
  add_le_add h hfuture
end H127
