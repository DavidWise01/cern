import Mathlib
namespace H171
theorem flux_partition (q w h : ℚ) :
 q*w + q*h + q*(1-w-h) = q := by ring
theorem zero_flux_zero_work (w : ℚ) : (0:ℚ)*w=0 := by ring
theorem ledger (s w h e q : ℚ) (h₁ : s+w+h+e=q) : s+w+h+e=q := h₁
end H171
