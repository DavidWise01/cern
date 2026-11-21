import Mathlib
namespace H182
-- If every observation's propagated interval encloses the current state,
-- their intersection also encloses it.
theorem intersection_preserves_coverage
    (x l₁ h₁ l₂ h₂ : ℚ)
    (h₁l : l₁ ≤ x) (h₁h : x ≤ h₁)
    (h₂l : l₂ ≤ x) (h₂h : x ≤ h₂) :
    max l₁ l₂ ≤ x ∧ x ≤ min h₁ h₂ := by
  constructor
  · exact max_le h₁l h₂l
  · exact le_min h₁h h₂h

theorem narrowing_never_widens
    (l₁ h₁ l₂ h₂ : ℚ) :
    min h₁ h₂ - max l₁ l₂ ≤ h₁ - l₁ := by
  have a := min_le_left h₁ h₂
  have b := le_max_left l₁ l₂
  linarith

theorem conservative_update (phase drift correction : ℚ) :
    (phase + drift - correction) + correction = phase + drift := by ring
end H182
