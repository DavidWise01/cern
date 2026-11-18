import Mathlib
namespace H110
theorem reserve_conservation (used request budget : ℝ)
    (h : used + request ≤ budget) :
    budget - (used + request) ≥ 0 := by linarith
theorem double_spend_counterexample :
    (3 * (3/5 : ℝ) > 1) ∧ (3/5 : ℝ) ≤ 1 := by norm_num
theorem terminal_containment (base used r : ℝ)
    (h : used ≤ r - base) : base + used ≤ r := by linarith
end H110
