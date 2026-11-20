import Mathlib
namespace H139
theorem cancellation (g gyro a ev e : ℝ) :
    g-gyro+(a-g+gyro-2*ev-e)-a = -2*ev-e := by ring
theorem clearance (r tube distance : ℝ) (h : tube < distance) (hr : r ≤ tube) :
    r < distance := lt_of_le_of_lt hr h
end H139
