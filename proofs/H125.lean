import Mathlib
namespace H125
theorem antisymmetric_recovery (a b q bias : ℝ) :
    ((a+b)/2+q+bias-((a+b)/2-q+bias))/2 = q := by ring
theorem simultaneous_error (ep em eps : ℝ)
    (hp : |ep| ≤ eps) (hm : |em| ≤ eps) :
    |(ep-em)/2| ≤ eps := by
  have hp' := abs_le.mp hp
  have hm' := abs_le.mp hm
  apply abs_le.mpr
  constructor <;> linarith [hp'.1,hp'.2,hm'.1,hm'.2]
end H125
