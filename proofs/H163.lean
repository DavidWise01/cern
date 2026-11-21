import Mathlib
namespace H163
theorem conserve_total (a b : ℚ) :
 (3*a+b)/4 + (a+3*b)/4 = a+b := by ring
theorem halve_difference (a b : ℚ) :
 (3*a+b)/4 - (a+3*b)/4 = (a-b)/2 := by ring
theorem energy_contraction (delta x : ℝ) :
 (delta/2)^2 + ((x+delta)/4)^2 ≤ (3/8)*(delta^2+x^2) := by nlinarith [sq_nonneg (delta-x)]
end H163
