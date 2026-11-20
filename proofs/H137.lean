import Mathlib
namespace H137
theorem routh_fixture : 27 * (1/100 : ℚ) * (1-1/100) < 1 := by norm_num
theorem routh_counterfixture : 1 < 27 * (1/20 : ℚ) * (1-1/20) := by norm_num
theorem gyro_no_work (x y : ℝ) : x*(-y)+y*x=0 := by ring
end H137
