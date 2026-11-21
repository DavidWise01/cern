import Mathlib
namespace H169
theorem angular_ledger (lr lh le j : ℚ) :
 (lr+j)+(lh-j)+le = lr+lh+le := by ring
theorem energy_ledger (fuel oldK newK work heat : ℚ) :
 (fuel-(newK-oldK)) + newK + work + heat
 = fuel+oldK+work+heat := by ring
theorem external_reaction (lr next_lr le : ℚ) :
 next_lr+(le+lr-next_lr)=lr+le := by ring
end H169
