import Mathlib
namespace H092
variable {G : Type*} [Group G]
theorem shell_cycle (a b c x y z : G) :
    (x*c*z⁻¹)*(z*b*y⁻¹)*(y*a*x⁻¹) =
      x*(c*b*a)*x⁻¹ := by
  group
theorem inverse_shell (d : G) : d*d⁻¹ = 1 := mul_inv_cancel d
end H092
