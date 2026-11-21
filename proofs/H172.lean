import Mathlib
namespace H172
theorem coupling_partition (q k : ℚ) :
 q*k+q*(1-k)=q := by ring
theorem conversion_partition (coupled eta : ℚ) :
 coupled*eta+coupled*(1-eta)=coupled := by ring
theorem extraction_partition (rotor load : ℚ) :
 rotor*load+rotor*(1-load)=rotor := by ring
theorem drag_partition (remaining : ℚ) :
 remaining*(15/16 : ℚ)+remaining*(1/16 : ℚ)=remaining := by ring
end H172
