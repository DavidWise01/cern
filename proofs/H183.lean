import Mathlib
namespace H183
theorem observation_residual (actual predicted noise : ℚ)
    (h : actual = predicted + noise) :
    actual - predicted = noise := by linarith
theorem true_delay_not_excluded (noise bound : ℚ)
    (h : |noise| ≤ bound) : |noise| ≤ bound := h
theorem known_pulse_difference (theta drift pulse : ℚ) :
    (theta + drift - pulse) - (theta + drift) = -pulse := by ring
end H183
