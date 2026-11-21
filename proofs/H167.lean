import Mathlib
namespace H167
theorem balanced_linear_impulse (j : ℚ) : j + (-j) = 0 := by ring
theorem same_signed_torques (R F : ℚ) : R*F + (-R)*(-F) = 2*R*F := by ring
theorem angular_reaction (L : ℚ) : L+(-L)=0 := by ring
theorem energy_ledger (E kold knew : ℚ) :
 (E - (knew - kold)) + knew = E + kold := by ring
end H167
