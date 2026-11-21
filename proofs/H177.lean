import Mathlib
namespace H177
theorem common_shift_preserves_difference (a b shift : ℚ) :
 (a+shift)-(b+shift)=a-b := by ring
theorem mean_feedback_contracts (a b k m : ℚ) :
 ((1-k)*a+k*m)-((1-k)*b+k*m)=(1-k)*(a-b) := by ring
theorem accounting (initial budget spent : ℚ)
  (h : budget+spent=initial) : initial-budget=spent := by linarith
end H177
