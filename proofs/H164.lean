import Mathlib
namespace H164
theorem quench_conserves (u s r : ℚ) : (3*u/4) + (s+u/4) + r = u+s+r := by ring
theorem release_conserves (u s r : ℚ) : u + s/2 + (r+s/2) = u+s+r := by ring
end H164
