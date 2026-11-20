import Mathlib
namespace H134
theorem common_bound (a worst e h : ℝ)
    (ha : a ≤ worst) (hw : worst+e ≤ h) :
    a+e ≤ h := by linarith
theorem switch_counterexample : (505 : ℚ)/504 > 1 := by norm_num
end H134
