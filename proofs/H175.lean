import Mathlib
namespace H175
theorem impulse_and_housing (rotor housing impulse : ℚ) :
 (rotor + impulse) + (housing - impulse) = rotor + housing := by ring
theorem extraction_and_receiver (rotor receiver impulse : ℚ) :
 (rotor - impulse) + (receiver + impulse) = rotor + receiver := by ring
theorem energy_account (fuel oldK newK exhaust heat : ℚ) :
 (fuel - (newK - oldK + exhaust)) + newK + exhaust + heat
 = fuel + oldK + heat := by ring
theorem opposite_signed_energy (l inertia : ℚ) :
 (-l)^2 / (2*inertia) = l^2 / (2*inertia) := by ring
end H175
