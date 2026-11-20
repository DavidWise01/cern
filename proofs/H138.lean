import Mathlib
namespace H138
theorem gyro_no_work (a b c x y z : ℝ) :
    x*(b*z-c*y)+y*(c*x-a*z)+z*(a*y-b*x)=0 := by ring
end H138
