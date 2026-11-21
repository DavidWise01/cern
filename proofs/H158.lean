import Mathlib
namespace H158
theorem three_independent_sources (w₁ w₂ w₃ : ℚ)
  (h₁ : (2/5 : ℚ) ≤ w₁) (h₂ : (2/5 : ℚ) ≤ w₂)
  (h₃ : (2/5 : ℚ) ≤ w₃) :
  (4/5 : ℚ) ≤ 2*w₁ ∧ (4/5 : ℚ) ≤ 2*w₂ ∧ (4/5 : ℚ) ≤ 2*w₃ := by
  constructor <;> try constructor <;> linarith
theorem three_bridge_min_total (w₁ w₂ w₃ : ℚ)
  (h₁ : (2/5 : ℚ) ≤ w₁) (h₂ : (2/5 : ℚ) ≤ w₂)
  (h₃ : (2/5 : ℚ) ≤ w₃) :
  (6/5 : ℚ) ≤ w₁+w₂+w₃ := by linarith
end H158
