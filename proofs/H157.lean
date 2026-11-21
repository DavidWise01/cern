import Mathlib
namespace H157
-- Independent roots: each bridge must meet the same target after
-- the opposite root is removed. The target is 4/5, so each >=2/5.
theorem each_bridge_weight (w : ℚ) (hw : (4/5:ℚ) ≤ 2*w) : (2/5:ℚ) ≤ w := by linarith
theorem two_bridge_budget (w₁ w₂ : ℚ) (h₁ : (2/5:ℚ) ≤ w₁)
    (h₂ : (2/5:ℚ) ≤ w₂) : (4/5:ℚ) ≤ w₁+w₂ := by linarith
end H157
