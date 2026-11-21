import Mathlib
namespace H165
theorem quench_ledger (u s e : ℚ) : 3*u/4+(s+u/4)+e=u+s+e := by ring
theorem release_ledger (u s e : ℚ) : u+s/2+(e+s/2)=u+s+e := by ring
theorem comoving_distance (a chi : ℚ) : a*chi=a*chi := rfl
theorem radiation_density (a energy : ℚ) (h : a ≠ 0) :
 (energy/a^4)*a^4=energy := by
 field_simp
end H165
